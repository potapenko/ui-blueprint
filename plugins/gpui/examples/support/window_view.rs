use gpui_kit::{
    component::{
        ActiveTheme,
        button::Button,
        input::{Input, InputState},
    },
    prelude::*,
    *,
};
use std::{path::PathBuf, time::Duration};
use uiblueprint_gpui::{Request, Session};

pub struct WindowView {
    enabled: bool,
    output: PathBuf,
    next_capture: u64,
    wide_gap: bool,
    long_text: bool,
    details: bool,
    overlay: bool,
    rows: Vec<u64>,
    next_row: u64,
    input: Entity<InputState>,
    scroll: ScrollHandle,
}
fn mark(probe: &Option<Session>, id: &str, element: impl IntoElement) -> AnyElement {
    match probe {
        Some(session) => {
            let (element, status) = session.mark_or_render(id, element);
            if let Err(gap) = status {
                eprintln!("{gap:?}");
            }
            element
        }
        None => element.into_any_element(),
    }
}
impl WindowView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        enabled: bool,
        output: PathBuf,
    ) -> Self {
        Self {
            enabled,
            output,
            next_capture: 0,
            wide_gap: false,
            long_text: false,
            details: false,
            overlay: false,
            rows: (1..=8).collect(),
            next_row: 9,
            input: cx.new(|cx| InputState::new(window, cx).placeholder("Type to verify input")),
            scroll: ScrollHandle::new(),
        }
    }
    fn capture(&mut self, component: Option<&str>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.enabled {
            println!("probe disabled");
            return;
        }
        let session = match Session::for_window(window, cx) {
            Ok(session) => session,
            Err(error) => {
                println!("refused {error}");
                return;
            }
        };
        self.next_capture += 1;
        let name = format!("capture-{}", self.next_capture);
        let mut request = Request::capture(&name, &name, Duration::from_secs(2), 1_048_576);
        if let Some(id) = component {
            request = request.component(id);
        }
        if let Err(error) = session.request(request, window) {
            println!("refused {error}");
            return;
        }
        let output = self.output.clone();
        window.on_next_frame(move |window, _| {
            window.on_next_frame(move |window, _| {
                let result = session
                    .take(window)
                    .unwrap_or(Err(uiblueprint_gpui::Error::IncompleteScope));
                match result.and_then(|capture| capture.to_json(&session, window)) {
                    Ok(bytes) => {
                        use std::io::Write;
                        let result = std::fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(output.join(format!("{name}.json")))
                            .and_then(|mut file| file.write_all(&bytes));
                        match result {
                            Ok(()) => println!(
                                "captured {name} {} bytes bounds={:?} viewport={:?} scale={}",
                                bytes.len(),
                                window.bounds(),
                                window.viewport_size(),
                                window.scale_factor()
                            ),
                            Err(_) => println!("refused output IO"),
                        }
                    }
                    Err(error) => {
                        session.cancel();
                        let _ = session.take(window);
                        println!("refused {error}");
                    }
                }
            })
        });
    }
    pub fn command(&mut self, line: &str, window: &mut Window, cx: &mut Context<Self>) {
        match line.trim() {
            "capture" => self.capture(None, window, cx),
            "capture toolbar" => self.capture(Some("toolbar"), window, cx),
            "quit" => cx.quit(),
            _ => println!("unknown command"),
        }
    }
}
impl Render for WindowView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let probe = self
            .enabled
            .then(|| Session::for_window(window, cx).expect("registered window root"));
        let toolbar = mark(
            &probe,
            "toolbar",
            div()
                .id("toolbar")
                .flex()
                .flex_wrap()
                .gap_2()
                .child(mark(
                    &probe,
                    "gap_control",
                    div()
                        .id("gap-frame")
                        .child(Button::new("gap").label("Change gap").on_click(cx.listener(
                            |v, _, _, cx| {
                                v.wide_gap = !v.wide_gap;
                                cx.notify();
                            },
                        ))),
                ))
                .child(mark(
                    &probe,
                    "text_control",
                    div().id("text-frame").child(
                        Button::new("text")
                            .label("Change text")
                            .on_click(cx.listener(|v, _, _, cx| {
                                v.long_text = !v.long_text;
                                cx.notify();
                            })),
                    ),
                ))
                .child(mark(
                    &probe,
                    "capture_control",
                    div().id("capture-frame").child(
                        Button::new("capture")
                            .label("Capture window")
                            .on_click(cx.listener(|v, _, w, cx| v.capture(None, w, cx))),
                    ),
                ))
                .child(mark(
                    &probe,
                    "narrow_control",
                    div().id("narrow-frame").child(
                        Button::new("capture-toolbar")
                            .label("Capture toolbar")
                            .on_click(cx.listener(|v, _, w, cx| v.capture(Some("toolbar"), w, cx))),
                    ),
                ))
                .child(mark(
                    &probe,
                    "add_control",
                    div()
                        .id("add-frame")
                        .child(Button::new("add").label("Add row").on_click(cx.listener(
                            |v, _, _, cx| {
                                if v.rows.len() < 32 {
                                    v.rows.insert(0, v.next_row);
                                    v.next_row += 1;
                                    cx.notify();
                                }
                            },
                        ))),
                ))
                .child(mark(
                    &probe,
                    "remove_control",
                    div().id("remove-frame").child(
                        Button::new("remove")
                            .label("Remove row")
                            .on_click(cx.listener(|v, _, _, cx| {
                                if !v.rows.is_empty() {
                                    v.rows.remove(0);
                                    cx.notify();
                                }
                            })),
                    ),
                ))
                .child(mark(
                    &probe,
                    "overlay_control",
                    div().id("overlay-frame").child(
                        Button::new("overlay")
                            .label(if self.overlay {
                                "Hide overlay"
                            } else {
                                "Show overlay"
                            })
                            .on_click(cx.listener(|v, _, _, cx| {
                                v.overlay = !v.overlay;
                                cx.notify();
                            })),
                    ),
                )),
        );
        // Exact px is the controlled diagnostic stimulus, not product spacing.
        let heading = mark(
            &probe,
            "heading",
            div()
                .id("heading")
                .flex()
                .items_center()
                .gap(px(if self.wide_gap { 18. } else { 8. }))
                .child(mark(
                    &probe,
                    "icon",
                    div()
                        .id("icon")
                        .size_8()
                        .bg(cx.theme().muted)
                        .rounded_md()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child("●"),
                ))
                .child(mark(
                    &probe,
                    "title",
                    div().id("title").text_lg().child("Window geometry"),
                )),
        );
        let body = if self.long_text {
            "A longer description wraps as the window changes size. Every marked region is included by the same default request, including new rows and an overlay rendered in the deferred pass."
        } else {
            "One request captures the registered window root and marked descendants."
        };
        let workspace = mark(
            &probe,
            "workspace",
            div()
                .id("workspace")
                .flex()
                .flex_col()
                .gap_3()
                .p_4()
                .border_1()
                .border_color(cx.theme().border)
                .rounded_lg()
                .child(heading)
                .child(mark(&probe, "body", div().id("body").text_sm().child(body)))
                .child(mark(
                    &probe,
                    "details_control",
                    div().id("details-frame").child(
                        Button::new("details")
                            .label(if self.details {
                                "Hide details"
                            } else {
                                "Show details"
                            })
                            .on_click(cx.listener(|v, _, _, cx| {
                                v.details = !v.details;
                                cx.notify();
                            })),
                    ),
                ))
                .when(self.details, |view| {
                    view.child(mark(
                        &probe,
                        "details",
                        div()
                            .id("details")
                            .p_2()
                            .bg(cx.theme().muted)
                            .child("This optional branch was not in a capture list."),
                    ))
                }),
        );
        let rows = self
            .rows
            .iter()
            .enumerate()
            .map(|(ix, id)| {
                mark(
                    &probe,
                    "row",
                    div()
                        .id(format!("private-row-{id}"))
                        .h_12()
                        .flex_shrink_0()
                        .p_2()
                        .child(mark(
                            &probe,
                            "row_label",
                            div().id("label").child(format!("Row {}", ix + 1)),
                        )),
                )
            })
            .collect::<Vec<_>>();
        let rows = mark(
            &probe,
            "rows",
            div()
                .id("rows")
                .h_40()
                .w_full()
                .overflow_y_scroll()
                .track_scroll(&self.scroll)
                .child(div().flex().flex_col().children(rows)),
        );
        mark(
            &probe,
            "content",
            div()
                .id("content")
                .size_full()
                .relative()
                .p_4()
                .flex()
                .flex_col()
                .gap_4()
                .bg(cx.theme().background)
                .text_color(cx.theme().foreground)
                .child(toolbar)
                .child(workspace)
                .child(rows)
                .child(mark(
                    &probe,
                    "input_frame",
                    div()
                        .id("input-frame")
                        .child(Input::new(&self.input).id("input")),
                ))
                .child(mark(
                    &probe,
                    "footer",
                    div()
                        .id("footer")
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child("UI Blueprint · default window root"),
                ))
                .when(self.overlay, |view| {
                    view.child(deferred(mark(
                        &probe,
                        "overlay",
                        div()
                            .id("overlay-layer")
                            .absolute()
                            .top_20()
                            .left_12()
                            .right_12()
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .bg(cx.theme().popover)
                            .border_1()
                            .border_color(cx.theme().border)
                            .rounded_lg()
                            .child(mark(
                                &probe,
                                "overlay_text",
                                div()
                                    .id("overlay-text")
                                    .child("Measured in the same window's deferred prepaint."),
                            ))
                            .child(mark(
                                &probe,
                                "overlay_close",
                                div().id("overlay-close-frame").child(
                                    Button::new("close-overlay")
                                        .label("Close overlay")
                                        .on_click(cx.listener(|v, _, _, cx| {
                                            v.overlay = false;
                                            cx.notify();
                                        })),
                                ),
                            )),
                    )))
                }),
        )
    }
}
