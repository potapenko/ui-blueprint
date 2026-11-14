#[path = "../examples/support/view.rs"]
#[allow(dead_code)] // Native stdin/output helpers are exercised by the native example.
mod support;
use gpui_kit::test::TestWindowExt;
use gpui_kit::{AppContext, Bounds, TestAppContext, Window, WindowBounds, WindowOptions, px, size};
use std::time::Duration;
use uiblueprint_gpui::{Capture, Error, Request};
use uiblueprint_schema::model::{Artifact, Document, Field, Shape, Value};

fn snapshot(capture: &Capture, view: &support::ExampleView, window: &Window) -> Document {
    let bytes = capture
        .to_json(view.session.as_ref().unwrap(), window)
        .expect("current publication");
    Document::from_json(&bytes, 262_144).expect("canonical snapshot")
}
fn rect(document: &Document, marker: &str) -> [f64; 4] {
    let Artifact::Snapshot(snapshot) = &document.artifact else {
        panic!("snapshot")
    };
    let node = snapshot
        .nodes
        .iter()
        .find(|node| node.key.key.0.starts_with(&format!("{marker}-")))
        .expect("marker");
    let Some(Value::Geometry(geometry)) = node
        .properties
        .iter()
        .find(|p| p.field() == Field::LayoutBounds)
        .and_then(|p| p.known())
    else {
        panic!("layout")
    };
    let Shape::Rect(r) = &geometry.shape else {
        panic!("rect")
    };
    [r.x, r.y, r.width, r.height]
}
fn capture(
    view: &gpui_kit::Entity<support::ExampleView>,
    window: &mut Window,
    cx: &mut gpui_kit::App,
) -> Capture {
    let session = view.read(cx).session.clone().unwrap();
    session
        .request(
            Request::new(
                "capture",
                "fixture",
                &view.read(cx).selected(),
                Duration::from_secs(2),
                262_144,
            ),
            window,
        )
        .expect("request");
    window.render_frame(cx);
    session
        .take(window)
        .expect("completed frame")
        .expect("captured")
}

#[gpui_kit::test]
fn resolved_layout_delta_remount_and_refusals(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let (handle, view) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(720.), px(640.)),
                ))),
                ..Default::default()
            },
            cx,
            |window, cx| {
                cx.new(|cx| {
                    support::ExampleView::new(
                        window,
                        cx,
                        true,
                        "headless-run",
                        "headless-fixture",
                        None,
                    )
                })
            },
        )
        .unwrap()
    });
    cx.update_window(handle, |_, window, cx| {
        window.render_frame(cx);
        let base = capture(&view, window, cx);
        let doc = snapshot(&base, view.read(cx), window);
        let icon = rect(&doc, "icon");
        let title = rect(&doc, "title");
        assert_eq!(title[0] - icon[0] - icon[2], 8.0);
        let Artifact::Snapshot(s) = doc.artifact else {
            unreachable!()
        };
        assert_eq!(s.nodes.len(), 11);
        assert!(s.nodes.iter().all(|n| {
            n.properties
                .iter()
                .filter(|p| p.field() != Field::LayoutBounds)
                .all(|p| p.known().is_none())
        }));
        assert!(
            !String::from_utf8(
                base.to_json(view.read(cx).session.as_ref().unwrap(), window)
                    .unwrap()
            )
            .unwrap()
            .contains("Layout sample")
        );
        window.click("gap", cx);
        assert!(view.read(cx).wide_gap);
        assert_eq!(
            base.to_json(view.read(cx).session.as_ref().unwrap(), window),
            Err(Error::StaleTarget)
        );
        let changed = capture(&view, window, cx);
        let doc = snapshot(&changed, view.read(cx), window);
        let icon = rect(&doc, "icon");
        let title = rect(&doc, "title");
        assert_eq!(title[0] - icon[0] - icon[2], 18.0);
        window.click("details", cx);
        let opened = capture(&view, window, cx);
        let old_mount = opened.mount("detail").unwrap();
        // Replace the native keyed child in adjacent frames: no missing frame.
        view.update(cx, |v, cx| {
            v.detail_epoch += 1;
            cx.notify();
        });
        let session = view.read(cx).session.clone().unwrap();
        session
            .request(
                Request::new(
                    "key-replaced",
                    "fixture",
                    &view.read(cx).selected(),
                    Duration::from_secs(2),
                    262_144,
                )
                .with_expected_mount("detail", old_mount),
                window,
            )
            .unwrap();
        window.render_frame(cx);
        assert!(matches!(
            session.take(window),
            Some(Err(Error::StaleTarget))
        ));
        let replacement = capture(&view, window, cx);
        assert_ne!(replacement.mount("detail"), Some(old_mount));
        let old_mount = replacement.mount("detail").unwrap();
        window.click("details", cx);
        let closed = capture(&view, window, cx);
        assert!(closed.mount("detail").is_none());
        window.click("details", cx);
        let reopened = capture(&view, window, cx);
        assert_ne!(reopened.mount("detail"), Some(old_mount));
        let session = view.read(cx).session.clone().unwrap();
        session
            .request(
                Request::new(
                    "stale",
                    "fixture",
                    &view.read(cx).selected(),
                    Duration::from_secs(2),
                    262_144,
                )
                .with_expected_mount("detail", old_mount),
                window,
            )
            .unwrap();
        window.render_frame(cx);
        assert!(matches!(
            session.take(window),
            Some(Err(Error::StaleTarget))
        ));
        session
            .request(
                Request::new(
                    "limit",
                    "fixture",
                    &view.read(cx).selected(),
                    Duration::from_secs(2),
                    8,
                ),
                window,
            )
            .unwrap();
        window.render_frame(cx);
        let small = session.take(window).unwrap().unwrap();
        assert_eq!(small.to_json(&session, window), Err(Error::Limit));
        view.update(cx, |v, cx| {
            v.duplicate = true;
            cx.notify();
        });
        session
            .request(
                Request::new(
                    "duplicate",
                    "fixture",
                    &view.read(cx).selected(),
                    Duration::from_secs(2),
                    262_144,
                ),
                window,
            )
            .unwrap();
        window.render_frame(cx);
        assert!(matches!(session.take(window), Some(Err(Error::Duplicate))));
        session
            .request(
                Request::new(
                    "cancel",
                    "fixture",
                    &view.read(cx).selected(),
                    Duration::from_secs(2),
                    262_144,
                ),
                window,
            )
            .unwrap();
        session.cancel();
        assert!(matches!(session.take(window), Some(Err(Error::Cancelled))));
        view.update(cx, |v, cx| {
            v.duplicate = false;
            v.expanded = false;
            cx.notify();
        });
        session
            .request(
                Request::new(
                    "missing",
                    "fixture",
                    &["root", "card", "detail"],
                    Duration::from_secs(2),
                    262_144,
                ),
                window,
            )
            .unwrap();
        window.render_frame(cx);
        assert!(matches!(
            session.take(window),
            Some(Err(Error::IncompleteScope))
        ));
        assert_eq!(
            session.request(
                Request::new(
                    "ancestor-missing",
                    "fixture",
                    &["root", "detail"],
                    Duration::from_secs(2),
                    262_144
                ),
                window
            ),
            Err(Error::InvalidScope)
        );
        session
            .request(
                Request::new(
                    "deadline",
                    "fixture",
                    &["root"],
                    Duration::from_millis(1),
                    262_144,
                ),
                window,
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(3));
        assert!(matches!(session.take(window), Some(Err(Error::Deadline))));
        session.close();
        assert_eq!(
            session.request(
                Request::new(
                    "closed",
                    "fixture",
                    &["root"],
                    Duration::from_secs(2),
                    262_144
                ),
                window
            ),
            Err(Error::StaleTarget)
        );
    })
    .unwrap();
}

#[gpui_kit::test]
fn sessions_in_the_same_run_have_distinct_saved_identity(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let mut windows = Vec::new();
    for _ in 0..2 {
        windows.push(cx.update(|cx| {
            gpui_kit::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        Default::default(),
                        size(px(720.), px(640.)),
                    ))),
                    ..Default::default()
                },
                cx,
                |window, cx| {
                    cx.new(|cx| {
                        support::ExampleView::new(window, cx, true, "same-app-run", "fixture", None)
                    })
                },
            )
            .unwrap()
        }));
    }
    let (first_handle, first_view) = &windows[0];
    let (first_capture, first) = cx
        .update_window(*first_handle, |_, window, cx| {
            let captured = capture(first_view, window, cx);
            let doc = snapshot(&captured, first_view.read(cx), window);
            (captured, doc)
        })
        .unwrap();
    let (second_handle, second_view) = &windows[1];
    let (second_capture, second) = cx
        .update_window(*second_handle, |_, window, cx| {
            let captured = capture(second_view, window, cx);
            let doc = snapshot(&captured, second_view.read(cx), window);
            let session = second_view.read(cx).session.clone().unwrap();
            session
                .request(
                    Request::new(
                        "wrong-window-generation",
                        "fixture",
                        &second_view.read(cx).selected(),
                        Duration::from_secs(2),
                        262_144,
                    )
                    .with_expected_surface(first_capture.surface_generation()),
                    window,
                )
                .unwrap();
            window.render_frame(cx);
            assert!(matches!(
                session.take(window),
                Some(Err(Error::StaleTarget))
            ));
            (captured, doc)
        })
        .unwrap();
    assert_ne!(
        first_capture.surface_generation(),
        second_capture.surface_generation()
    );
    assert_ne!(first_capture.mount("root"), second_capture.mount("root"));
    let (Artifact::Snapshot(first), Artifact::Snapshot(second)) = (first.artifact, second.artifact)
    else {
        panic!("snapshots")
    };
    assert_eq!(first.context.target, second.context.target);
    assert_ne!(first.context.session_id, second.context.session_id);
    assert_ne!(first.context.surfaces, second.context.surfaces);
    assert_ne!(first.id, second.id);
    assert_ne!(
        first.observations[0].clock_domain,
        second.observations[0].clock_domain
    );
}
