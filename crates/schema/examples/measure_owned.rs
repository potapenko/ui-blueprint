//! Explicit-input D05 sizing probe. No UI collection or production policy.
use uiblueprint_schema::owned_size as memory;

use std::{
    env,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use uiblueprint_schema::model::*;

fn read(path: &Path, limit: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut data = Vec::new();
    let bound = u64::try_from(limit.checked_add(1).ok_or("input limit overflow")?)?;
    File::open(path)?.take(bound).read_to_end(&mut data)?;
    if data.len() > limit {
        return Err("input exceeded explicit byte limit".into());
    }
    Ok(data)
}
fn size<T: memory::HeapSize>(value: &T) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let (inline, heap, total) = memory::owned(value).map_err(|_| "owned-size overflow")?;
    Ok(
        serde_json::json!({"inline_bytes":inline,"heap_capacity_bytes":heap.bytes,"owned_bytes":total,"allocation_blocks":heap.allocations,"unused_capacity_bytes":heap.slack_bytes}),
    )
}
fn snapshots(d: &Document) -> Vec<(&'static str, &Snapshot)> {
    match &d.artifact {
        Artifact::Snapshot(s) => vec![("snapshot", s)],
        Artifact::ChannelResponse(r) => match &r.result {
            ChannelResult::Observed(s) => vec![("observed", s)],
            ChannelResult::Failed(_) => vec![],
        },
        Artifact::Delta(d) => std::iter::once(("base", &d.base))
            .chain(d.source_snapshot.as_ref().map(|s| ("source_snapshot", s)))
            .collect(),
        Artifact::Action(a) => vec![("snapshot", &a.snapshot)],
        Artifact::Finding(f) => vec![("snapshot", &f.snapshot)],
        Artifact::GoldenChain(c) => vec![("before", &c.before), ("after", &c.after)],
        Artifact::ActionResult(a) => vec![("snapshot", &a.snapshot)],
        Artifact::DeltaResult(d) => d
            .base
            .as_ref()
            .map(|s| vec![("base", s)])
            .unwrap_or_default(),
        Artifact::TransitionContext(t) => std::iter::once(("before", &t.before))
            .chain(t.after.as_ref().map(|s| ("after", s)))
            .collect(),
        _ => vec![],
    }
}
fn row(
    cohort: &str,
    id: &str,
    bytes: &[u8],
    limit: usize,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let document = Document::from_json(bytes, limit)
        .map_err(|_| "sample failed current canonical validation")?;
    let mut measured = Vec::new();
    for (role, snapshot) in snapshots(&document) {
        measured.push(serde_json::json!({"role":role,"nodes":snapshot.nodes.len(),"relations":snapshot.relations.len(),"coverage":snapshot.coverage.status,"size":size(snapshot)?}));
    }
    Ok(
        serde_json::json!({"cohort":cohort,"sample":id,"serialized_input_bytes":bytes.len(),"document":size(&document)?,"snapshots":measured}),
    )
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args_os().skip(1).collect();
    if args.len() < 4 || args[0] != "--max-input-bytes" || args.len() % 2 != 0 {
        return Err(
            "usage: --max-input-bytes N --p1 DIR [--web-report FILE] [--document FILE]".into(),
        );
    }
    let limit = args[1].to_str().ok_or("invalid limit")?.parse::<usize>()?;
    if limit == 0 {
        return Err("zero input limit".into());
    }
    let mut rows = Vec::new();
    let mut rejected = 0;
    for pair in args[2..].chunks_exact(2) {
        let path = PathBuf::from(&pair[1]);
        match pair[0].to_str().ok_or("invalid option")? {
            "--p1" => {
                let manifest: Vec<serde_json::Value> =
                    serde_json::from_slice(&read(&path.join("manifest.json"), limit)?)?;
                for entry in manifest {
                    let name = entry["path"].as_str().ok_or("manifest path missing")?;
                    if Path::new(name).components().count() != 1 {
                        return Err("manifest path must be local filename".into());
                    }
                    if entry["validator_exit"] != 0 {
                        rejected += 1;
                        continue;
                    }
                    let bytes = read(&path.join(name), limit)?;
                    rows.push(row("p1", name, &bytes, limit)?);
                }
            }
            "--web-report" => {
                let report: serde_json::Value = serde_json::from_slice(&read(&path, limit)?)?;
                for sample in report["snapshots"].as_array().ok_or("snapshots missing")? {
                    let name = sample["state"].as_str().ok_or("state missing")?;
                    let bytes = serde_json::to_vec(
                        &serde_json::json!({"schema_version":"0.1.0","artifact":{"kind":"snapshot","data":sample["snapshot"]}}),
                    )?;
                    rows.push(row("web_b03_narrow", name, &bytes, limit)?);
                }
            }
            "--document" => {
                let bytes = read(&path, limit)?;
                rows.push(row(
                    "supplied_canonical_document",
                    &path.to_string_lossy(),
                    &bytes,
                    limit,
                )?);
            }
            _ => return Err("unsupported option".into()),
        }
    }
    let report = serde_json::json!({"method":"owned_capacity_walk_v1","os":env::consts::OS,"arch":env::consts::ARCH,"pointer_bits":usize::BITS,"excluded":"allocator headers/rounding, temporary parser/serializer/input buffers, harness, OS/SDK memory, external pixels and other payload_ref data","invalid_p1_records_not_retained":rejected,"samples":rows});
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
