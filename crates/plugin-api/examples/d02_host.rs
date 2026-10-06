//! Finite Node-facing proof wrapper. stdin contains canonical channel frames only.
#[path = "../../../tests/bridges/common/session_support.rs"]
mod support;

use std::{
    env,
    io::{self, Write},
    path::PathBuf,
    process::ExitCode,
    sync::mpsc::RecvTimeoutError,
    thread,
    time::Duration,
};
use support::{FrameEvent, Harness};
use uiblueprint_plugin_api::{Completion, Error, Limits};
use uiblueprint_schema::model::*;

struct Config {
    session: PathBuf,
    request: PathBuf,
    frame_bytes: usize,
    pending_bytes: usize,
    mode: String,
    late_ms: u64,
    max_frames: usize,
    retained: Option<PathBuf>,
}
fn config() -> Option<Config> {
    let a: Vec<_> = env::args_os().skip(1).collect();
    if a.len() != 8 {
        return None;
    }
    let frame_bytes = a[2].to_str()?.parse::<usize>().ok()?;
    let pending_bytes = a[3].to_str()?.parse::<usize>().ok()?;
    let mode = a[4].to_str()?.to_owned();
    if ![
        "complete",
        "cancel-after-first",
        "detach-after-first",
        "expire-after-first",
    ]
    .contains(&mode.as_str())
    {
        return None;
    }
    let late_ms = a[5].to_str()?.parse::<u64>().ok()?;
    let max_frames = a[6].to_str()?.parse::<usize>().ok()?;
    if frame_bytes == 0 || max_frames == 0 || late_ms == 0 {
        return None;
    }
    Some(Config {
        session: a[0].clone().into(),
        request: a[1].clone().into(),
        frame_bytes,
        pending_bytes,
        mode,
        late_ms,
        max_frames,
        retained: if a[7] == "-" {
            None
        } else {
            Some(a[7].clone().into())
        },
    })
}
fn emit(value: serde_json::Value) -> Result<(), ()> {
    let mut out = io::stdout().lock();
    serde_json::to_writer(&mut out, &value).map_err(|_| ())?;
    out.write_all(b"\n")
        .and_then(|_| out.flush())
        .map_err(|_| ())
}
fn terminal(h: &Harness, completion: Completion, cfg: &Config) -> Result<(), ()> {
    let terminal = format!("{:?}", completion.terminal);
    let missing = completion.missing_channels.clone();
    let documents = h.retained_documents(completion);
    let mut channels = Vec::new();
    for (index, document) in documents.iter().enumerate() {
        let Artifact::ChannelResponse(response) = &document.artifact else {
            return Err(());
        };
        let (status, nodes) = match &response.result {
            ChannelResult::Observed(s) => ("observed", s.nodes.len()),
            ChannelResult::Failed(_) => ("failed", 0),
        };
        channels.push(serde_json::json!({"channel":response.channel,"status":status,"nodes":nodes,"retained_file":cfg.retained.as_ref().map(|_| format!("channel-{index}.json"))}));
        if let Some(dir) = &cfg.retained {
            let file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(dir.join(format!("channel-{index}.json")))
                .map_err(|_| ())?;
            serde_json::to_writer(file, document).map_err(|_| ())?;
        }
    }
    emit(
        serde_json::json!({"event":"terminal","terminal":terminal,"mode":cfg.mode,"injected_control":cfg.mode!="complete","parent_elapsed_ms":h.elapsed_ms().map_err(|_| ())?,"channels":channels,"missing_channels":missing}),
    )
}
fn run(cfg: Config) -> Result<u8, ()> {
    let Artifact::Session(session) = support::read_document(&cfg.session, cfg.frame_bytes)
        .map_err(|_| ())?
        .artifact
    else {
        return Err(());
    };
    let Artifact::Request(request) = support::read_document(&cfg.request, cfg.frame_bytes)
        .map_err(|_| ())?
        .artifact
    else {
        return Err(());
    };
    // Proof safety ceiling, not a product timeout: prevents an accidental hours-
    // long test invocation. Normal callers supply D05's finite per-request limits.
    if request.limits.deadline_ms > 30_000 || cfg.late_ms > 5_000 {
        return Err(());
    }
    if let Some(dir) = &cfg.retained {
        std::fs::create_dir(dir).map_err(|_| ())?;
    }
    let mut h = Harness::new(
        *session,
        *request,
        Limits {
            max_frame_bytes: cfg.frame_bytes,
            max_in_flight: 1,
            max_pending_encoded_bytes: cfg.pending_bytes,
        },
    )
    .map_err(|_| ())?;
    emit(
        serde_json::json!({"event":"ticket","request_id":h.ticket().request_id,"session_id":h.ticket().session_id,"sequence":h.ticket().sequence,"parent_clock_domain":h.clock_domain(),"parent_elapsed_ms":h.elapsed_ms().map_err(|_| ())?}),
    )?;
    let (rx, _reader) = support::pump_frames(io::stdin(), cfg.frame_bytes, cfg.max_frames);
    loop {
        if h.remaining().map_err(|_| ())?.is_zero() {
            let done = h.expire().map_err(|_| ())?;
            terminal(&h, done, &cfg)?;
            return Ok(2);
        }
        match rx.recv_timeout(h.remaining().map_err(|_| ())?) {
            Ok(FrameEvent::Frame(frame)) => match h.receive(&frame) {
                Ok(()) => {
                    emit(
                        serde_json::json!({"event":"frame_accepted","bytes":frame.len(),"parent_elapsed_ms":h.elapsed_ms().map_err(|_| ())?}),
                    )?;
                    if cfg.mode != "complete" {
                        let completion = match cfg.mode.as_str() {
                            "cancel-after-first" => h.cancel().map_err(|_| ())?,
                            "detach-after-first" => h.detach().into_iter().next().ok_or(())?,
                            _ => {
                                thread::sleep(h.remaining().map_err(|_| ())?);
                                h.expire().map_err(|_| ())?
                            }
                        };
                        terminal(&h, completion, &cfg)?;
                        return match rx.recv_timeout(Duration::from_millis(cfg.late_ms)) {
                            Ok(FrameEvent::Frame(late)) => {
                                let valid_late=Document::from_json(&late,cfg.frame_bytes).ok().is_some_and(|d| matches!(d.artifact,Artifact::ChannelResponse(r) if r.request_id==h.ticket().request_id && r.session_id==h.ticket().session_id && r.dispatch_sequence==h.ticket().sequence && &r.target==h.target()));
                                if !valid_late {
                                    emit(
                                        serde_json::json!({"event":"late_probe_invalid","injected":true}),
                                    )?;
                                    return Ok(2);
                                }
                                let result = h.receive(&late);
                                let rejected = matches!(
                                    result,
                                    Err(Error::StaleTicket
                                        | Error::Detached
                                        | Error::DeadlineExpired)
                                );
                                emit(
                                    serde_json::json!({"event":"late_probe","injected":true,"rejected":rejected,"code":result.err().map(|e| format!("{e:?}"))}),
                                )?;
                                Ok(if rejected { 0 } else { 2 })
                            }
                            _ => {
                                emit(
                                    serde_json::json!({"event":"late_probe_missing","injected":true}),
                                )?;
                                Ok(2)
                            }
                        };
                    }
                    match h.complete() {
                        Ok(done) => {
                            terminal(&h, done, &cfg)?;
                            return Ok(0);
                        }
                        Err(Error::Incomplete) => (),
                        Err(Error::DeadlineExpired) => {
                            let done = h.expire().map_err(|_| ())?;
                            terminal(&h, done, &cfg)?;
                            return Ok(2);
                        }
                        Err(_) => return Err(()),
                    }
                }
                Err(error) => {
                    emit(
                        serde_json::json!({"event":"frame_rejected","code":format!("{error:?}")}),
                    )?;
                }
            },
            Ok(FrameEvent::Eof) | Err(RecvTimeoutError::Disconnected) => {
                let done = h.cancel().map_err(|_| ())?;
                terminal(&h, done, &cfg)?;
                return Ok(2);
            }
            Err(RecvTimeoutError::Timeout) => {
                let done = h.expire().map_err(|_| ())?;
                terminal(&h, done, &cfg)?;
                return Ok(2);
            }
            Ok(error) => {
                emit(serde_json::json!({"event":"framing_rejected","code":format!("{error:?}")}))?;
                let done = h.cancel().map_err(|_| ())?;
                terminal(&h, done, &cfg)?;
                return Ok(2);
            }
        }
    }
}
fn main() -> ExitCode {
    match config().ok_or(()).and_then(run) {
        Ok(code) => ExitCode::from(code),
        Err(()) => {
            let _ = emit(serde_json::json!({"event":"host_error","code":"invalid_config_or_io"}));
            ExitCode::from(1)
        }
    }
}
