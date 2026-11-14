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
use uiblueprint_gpui::{Capture, Configuration, Marker, Request, Session};

pub struct ExampleView {
    pub session: Option<Session>,
    pub expanded: bool,
    pub long_text: bool,
    pub wide_gap: bool,
    pub duplicate: bool,
    pub detail_epoch: usize,
    pub input: Entity<InputState>,
    pub scroll: ScrollHandle,
    pub last: Option<Capture>,
    pub output: Option<PathBuf>,
    next_capture: u64,
}

impl ExampleView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        enabled: bool,
        run: &str,
        build: &str,
        output: Option<PathBuf>,
    ) -> Self {
        let markers = [
            ("root", None),
            ("card", Some("root")),
            ("heading", Some("card")),
            ("icon", Some("heading")),
            ("title", Some("heading")),
            ("body", Some("card")),
            ("action", Some("card")),
            ("detail", Some("card")),
            ("scroll", Some("root")),
            ("row0", Some("scroll")),
            ("row1", Some("scroll")),
            ("input", Some("root")),
        ]
        .into_iter()
        .map(|(id, parent)| Marker::new(id, parent))
        .collect();
        let session = enabled.then(|| {
            Session::new(
                Configuration::new(
                    "org.uiblueprint.gpui-example",
                    build,
                    run,
                    "sample",
                    markers,
                ),
                window,
            )
            .expect("static diagnostic declarations")
        });
        Self {
            session,
            expanded: false,
            long_text: false,
            wide_gap: false,
            duplicate: false,
            detail_epoch: 0,
            input: cx.new(|cx| InputState::new(window, cx).placeholder("Type to verify input")),
            scroll: ScrollHandle::new(),
            last: None,
            output,
            next_capture: 0,
        }
    }

    pub fn selected(&self) -> Vec<&'static str> {
        let mut selected = vec![
            "root", "card", "heading", "icon", "title", "body", "action", "scroll", "row0", "row1",
            "input",
        ];
        if self.expanded {
            selected.push("detail");
        }
        selected
    }

    fn mark(&self, id: &str, element: impl IntoElement) -> AnyElement {
        match &self.session {
            Some(session) => session
                .mark(id, element)
                .expect("static diagnostic marker")
                .into_any_element(),
            None => element.into_any_element(),
        }
    }

    pub fn capture(&mut self, name: &str, window: &mut Window, cx: &mut Context<Self>) {
        if name.is_empty()
            || name.len() > 48
            || !name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
        {
            println!("refused invalid capture name");
            return;
        }
        let Some(session) = self.session.clone() else {
            println!("probe disabled");
            return;
        };
        let Some(output) = self.output.clone() else {
            return;
        };
        let request = Request::new(
            name,
            name,
            &self.selected(),
            Duration::from_secs(2),
            262_144,
        );
        if let Err(error) = session.request(request, window) {
            println!("refused {error}");
            return;
        }
        let name = name.to_owned();
        let view = cx.weak_entity();
        // In pinned GPUI0.3.12 native callbacks run BEFORE drawing (window.rs:2008).
        // The second frame event publishes after the one requested redraw, without
        // a second collection request or a polling loop.
        window.on_next_frame(move |window, _| {
            window.on_next_frame(move |window, cx| {
                let result = session
                    .take(window)
                    .unwrap_or(Err(uiblueprint_gpui::Error::IncompleteScope));
                match result.and_then(|capture| {
                    capture
                        .to_json(&session, window)
                        .map(|bytes| (capture, bytes))
                }) {
                    Ok((capture, bytes)) => {
                        use std::io::Write;
                        let path = output.join(format!("{name}.json"));
                        let result = std::fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(path)
                            .and_then(|mut file| file.write_all(&bytes));
                        match result {
                            Ok(()) => {
                                let bounds = window.bounds();
                                let viewport = window.viewport_size();
                                println!("window {name} bounds={bounds:?} viewport={viewport:?} scale={}", window.scale_factor());
                                println!("captured {name} {} bytes", bytes.len());
                                let _ = view.update(cx, |this, _| this.last = Some(capture));
                            }
                            Err(_) => println!("refused output IO"),
                        }
                    }
                    Err(error) => {
                        session.cancel();
                        let _ = session.take(window);
                        println!("refused {error}");
                    }
                }
            });
        });
    }

    pub fn command(&mut self, line: &str, window: &mut Window, cx: &mut Context<Self>) {
        let line = line.trim();
        if let Some(name) = line.strip_prefix("capture ") {
            self.capture(name, window, cx);
            return;
        }
        match line {
            "gap" => self.wide_gap = !self.wide_gap,
            "long" => self.long_text = !self.long_text,
            "toggle" => self.expanded = !self.expanded,
            "duplicate" => self.duplicate = !self.duplicate,
            "scroll" => self.scroll.set_offset(point(px(0.), px(-70.))),
            "top" => self.scroll.set_offset(point(px(0.), px(0.))),
            "narrow" => window.resize(size(px(480.), px(640.))),
            "wide" => window.resize(size(px(720.), px(640.))),
            "quit" => {
                cx.quit();
                return;
            }
            _ => {
                println!("unknown command");
                return;
            }
        }
        cx.notify();
        println!("changed {line}");
    }
}

impl Render for ExampleView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Exact pixel gap is the intentional controlled measurement stimulus, not client styling.
        let gap = if self.wide_gap { 18. } else { 8. };
        let heading = self.mark(
            "heading",
            div()
                .id("heading")
                .flex()
                .items_center()
                .gap(px(gap))
                .child(
                    self.mark(
                        "icon",
                        div()
                            .id("icon")
                            .size_8()
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(cx.theme().muted)
                            .rounded_md()
                            .child("●"),
                    ),
                )
                .child(self.mark("title", div().id("title").text_lg().child("Layout sample"))),
        );
        let body = if self.long_text {
            "A longer description wraps as the native window gets narrower. The collector must measure the resolved result without guessing from the text or its configured width. This is local diagnostic content."
        } else {
            "A native component with measured internal parts."
        };
        let card = self.mark(
            "card",
            div()
                .id("card")
                .flex()
                .flex_col()
                .p_4()
                .gap_3()
                .border_1()
                .border_color(cx.theme().border)
                .rounded_lg()
                .child(heading)
                .child(self.mark("body", div().id("body").text_sm().child(body)))
                .child(
                    self.mark(
                        "action",
                        div().id("action-frame").child(
                            Button::new("details")
                                .label(if self.expanded {
                                    "Hide details"
                                } else {
                                    "Show details"
                                })
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.expanded = !this.expanded;
                                    cx.notify();
                                })),
                        ),
                    ),
                )
                .when(self.expanded, |card| {
                    card.child(
                        self.mark(
                            "detail",
                            div()
                                .id(("detail", self.detail_epoch))
                                .p_2()
                                .bg(cx.theme().muted)
                                .child("The nested region is mounted."),
                        ),
                    )
                })
                .when(self.duplicate, |card| {
                    card.child(
                        self.mark("body", div().id("duplicate-body").child("Duplicate marker")),
                    )
                }),
        );
        let scroll = self.mark(
            "scroll",
            div()
                .id("scroll")
                .h_32()
                .w_full()
                .overflow_y_scroll()
                .track_scroll(&self.scroll)
                .child(div().flex().flex_col().children((0usize..8).map(|ix| {
                    let row = div()
                        .id(("row", ix))
                        .h_12()
                        .flex_shrink_0()
                        .p_2()
                        .child(format!("Scroll row {ix}"));
                    match ix {
                        0 => self.mark("row0", row),
                        1 => self.mark("row1", row),
                        _ => row.into_any_element(),
                    }
                }))),
        );
        self.mark(
            "root",
            div()
                .id("probe-root")
                .size_full()
                .p_4()
                .flex()
                .flex_col()
                .gap_4()
                .bg(cx.theme().background)
                .text_color(cx.theme().foreground)
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .child(Button::new("gap").label("Change gap").on_click(cx.listener(
                            |this, _, _, cx| {
                                this.wide_gap = !this.wide_gap;
                                cx.notify();
                            },
                        )))
                        .child(
                            Button::new("text")
                                .label("Change text")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.long_text = !this.long_text;
                                    cx.notify();
                                })),
                        ),
                )
                .child(
                    Button::new("capture")
                        .label("Capture")
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.next_capture += 1;
                            this.capture(&format!("capture-{}", this.next_capture), window, cx);
                        })),
                )
                .child(card)
                .child(scroll)
                .child(
                    self.mark(
                        "input",
                        div()
                            .id("input-frame")
                            .child(Input::new(&self.input).id("input")),
                    ),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(cx.theme().muted_foreground)
                        .child("UI Blueprint · GPUI geometry qualification"),
                ),
        )
    }
}
