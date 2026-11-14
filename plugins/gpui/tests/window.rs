use gpui_kit::{
    AnyWindowHandle, AppContext, Bounds, Context, Entity, IntoElement, Render, TestAppContext,
    Window, WindowBounds, WindowOptions, deferred, div, prelude::*, px, size,
};
use std::time::Duration;
use uiblueprint_gpui::{Configuration, Error, Request, Session, register};
use uiblueprint_schema::model::{Artifact, Document, Snapshot};

struct View {
    rows: Vec<u64>,
    extra: bool,
    overlay: bool,
}
impl Render for View {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let session = Session::for_window(window, cx).expect("registered actual Root");
        let rows = self
            .rows
            .iter()
            .map(|id| {
                let label = session
                    .mark("row_label", div().id("label").child("row"))
                    .unwrap();
                session
                    .mark(
                        "row",
                        div().id(format!("private-item-{id}")).h_6().child(label),
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let body = div()
            .id("content")
            .size_full()
            .flex()
            .flex_col()
            .child(session.mark("fixed", div().id("fixed").h_8()).unwrap())
            .children(rows)
            .when(self.extra, |body| {
                body.child(
                    session
                        .mark("dynamic", div().id("new-branch").h_8())
                        .unwrap(),
                )
            })
            .when(self.overlay, |body| {
                body.child(deferred(
                    session
                        .mark(
                            "overlay",
                            div()
                                .id("overlay")
                                .absolute()
                                .top_4()
                                .left_4()
                                .w_32()
                                .h_16()
                                .child(
                                    session
                                        .mark(
                                            "overlay_label",
                                            div().id("overlay-label").child("overlay"),
                                        )
                                        .unwrap(),
                                ),
                        )
                        .unwrap(),
                ))
            });
        session.mark("content", body).unwrap()
    }
}
fn setup(cx: &mut TestAppContext) -> (AnyWindowHandle, Entity<View>) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        register(
            Configuration::application("test.app", "fixture", "same-run")
                .required_anchors(&["content"]),
            cx,
        )
        .unwrap();
    });
    cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                    Default::default(),
                    size(px(640.), px(480.)),
                ))),
                ..Default::default()
            },
            cx,
            |_, cx| {
                cx.new(|_| View {
                    rows: vec![10, 20],
                    extra: false,
                    overlay: false,
                })
            },
        )
        .unwrap()
    })
}
fn capture(
    cx: &mut TestAppContext,
    handle: AnyWindowHandle,
    component: Option<&str>,
) -> Result<Snapshot, Error> {
    let session = cx
        .update_window(handle, |_, window, cx| {
            let session = Session::for_window(window, cx)?;
            let mut request =
                Request::capture("capture", "fixture", Duration::from_secs(2), 1_048_576);
            if let Some(id) = component {
                request = request.component(id);
            }
            session.request(request, window)?;
            window.render_frame(cx);
            // Root paint has happened, but the post-draw effect/retirement has not.
            assert!(session.take(window).is_none());
            Ok::<_, Error>(session)
        })
        .unwrap()?;
    cx.run_until_parked();
    cx.update_window(handle, |_, window, _| {
        let captured = session.take(window).ok_or(Error::IncompleteScope)??;
        let bytes = captured.to_json(&session, window)?;
        assert!(!String::from_utf8_lossy(&bytes).contains("private-item"));
        let document = Document::from_json(&bytes, 1_048_576).unwrap();
        let Artifact::Snapshot(snapshot) = document.artifact else {
            panic!("Snapshot")
        };
        Ok(*snapshot)
    })
    .unwrap()
}
fn markers(snapshot: &Snapshot, name: &str) -> Vec<usize> {
    snapshot.nodes.iter().enumerate().filter(|(_, node)| node.source_declarations.iter().any(|d|
        d.name.0 == "marker_id" && matches!(&d.state, uiblueprint_schema::model::Availability::Known { value: uiblueprint_schema::model::Value::Text(id) } if id == name)))
        .map(|(ix,_)| ix).collect()
}
use gpui_kit::test::TestWindowExt;

#[gpui_kit::test]
fn default_root_dynamic_occurrences_deferred_and_narrowing(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx);
    let first = capture(cx, handle, None).unwrap();
    assert_eq!(first.nodes.len(), 7);
    assert_eq!(markers(&first, "window_root").len(), 1);
    assert_eq!(markers(&first, "row").len(), 2);
    for ix in markers(&first, "row") {
        assert_eq!(first.nodes[ix].children.len(), 1);
    }
    assert_ne!(
        first.nodes[markers(&first, "row")[0]].children,
        first.nodes[markers(&first, "row")[1]].children
    );
    cx.update(|cx| {
        view.update(cx, |v, cx| {
            v.rows.push(30);
            v.extra = true;
            v.overlay = true;
            cx.notify();
        })
    });
    let second = capture(cx, handle, None).unwrap();
    assert_eq!(second.nodes.len(), 12);
    assert_eq!(markers(&second, "row").len(), 3);
    assert_eq!(markers(&second, "dynamic").len(), 1);
    let overlay = markers(&second, "overlay")[0];
    assert_eq!(second.nodes[overlay].children.len(), 1);
    let narrowed = capture(cx, handle, Some("content")).unwrap();
    assert_eq!(narrowed.nodes.len(), 11);
    assert!(markers(&narrowed, "window_root").is_empty());
    assert!(matches!(
        capture(cx, handle, Some("row")),
        Err(Error::AmbiguousTarget)
    ));
    cx.update(|cx| {
        view.update(cx, |v, cx| {
            v.rows.remove(0);
            v.extra = false;
            v.overlay = false;
            cx.notify();
        })
    });
    let removed = capture(cx, handle, None).unwrap();
    assert_eq!(removed.nodes.len(), 7);
    assert!(markers(&removed, "overlay").is_empty());
    assert!(
        !removed
            .nodes
            .iter()
            .any(|node| node.key == first.nodes[markers(&first, "row")[0]].key)
    );
}

#[gpui_kit::test]
fn window_identity_required_anchors_limits_and_bounded_churn(cx: &mut TestAppContext) {
    let (first, view) = setup(cx);
    let second = cx.update(|cx| {
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| View {
                rows: vec![],
                extra: false,
                overlay: false,
            })
        })
        .unwrap()
        .0
    });
    let old_session = cx
        .update_window(first, |_, window, cx| {
            Session::for_window(window, cx).unwrap()
        })
        .unwrap();
    cx.update_window(second, |_, window, _| {
        assert!(matches!(
            old_session.request(
                Request::capture("wrong", "fixture", Duration::from_secs(2), 1_048_576),
                window
            ),
            Err(Error::StaleTarget)
        ))
    })
    .unwrap();
    let before = capture(cx, first, None).unwrap();
    cx.update_window(first, |_, window, cx| {
        let replacement = cx.new(|_| View {
            rows: vec![77],
            extra: false,
            overlay: false,
        });
        window.replace_root(cx, |window, cx| {
            gpui_kit::base::Root::new(replacement, window, cx)
        });
        assert!(matches!(
            old_session.request(
                Request::capture("old-root", "fixture", Duration::from_secs(2), 1_048_576),
                window
            ),
            Err(Error::StaleTarget)
        ));
    })
    .unwrap();
    cx.run_until_parked();
    let after = capture(cx, first, None).unwrap();
    assert_ne!(before.context.session_id, after.context.session_id);
    assert_ne!(before.context.surfaces, after.context.surfaces);
    assert_eq!(
        capture(cx, first, Some("absent")).unwrap_err(),
        Error::IncompleteScope
    );
    // Existing second window is independently bound; replacement did not contaminate it.
    let other = capture(cx, second, None).unwrap();
    assert_ne!(after.context.surfaces, other.context.surfaces);
    drop(view);
}

#[gpui_kit::test]
fn repeated_dynamic_rows_retire_and_overflow_never_publishes(cx: &mut TestAppContext) {
    let (handle, view) = setup(cx);
    for generation in 0..140 {
        cx.update(|cx| {
            view.update(cx, |v, cx| {
                v.rows = vec![generation * 2 + 1, generation * 2 + 2];
                cx.notify();
            })
        });
        let snapshot = capture(cx, handle, None).unwrap();
        assert_eq!(snapshot.nodes.len(), 7);
    }
    cx.update(|cx| {
        view.update(cx, |v, cx| {
            v.rows = (0..130).collect();
            cx.notify();
        })
    });
    assert_eq!(capture(cx, handle, None).unwrap_err(), Error::Limit);
    cx.update(|cx| {
        view.update(cx, |v, cx| {
            v.rows = vec![800];
            cx.notify();
        })
    });
    assert_eq!(capture(cx, handle, None).unwrap().nodes.len(), 5);
}

struct CachedLeaf {
    renders: std::rc::Rc<std::cell::Cell<u32>>,
}
impl Render for CachedLeaf {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.renders.set(self.renders.get() + 1);
        Session::for_window(window, cx)
            .unwrap()
            .mark("cached_leaf", div().id("cached-leaf").size_full())
            .unwrap()
    }
}
struct CachedOwner {
    child: Entity<CachedLeaf>,
}
impl Render for CachedOwner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let style = div().w_32().h_8().style().clone();
        Session::for_window(window, cx)
            .unwrap()
            .mark(
                "content",
                div()
                    .id("content")
                    .size_full()
                    .child(self.child.clone().cached(style)),
            )
            .unwrap()
    }
}
#[gpui_kit::test]
fn explicit_request_revisits_actual_cached_subtree(cx: &mut TestAppContext) {
    cx.update(|cx| {
        gpui_kit::init(cx);
        register(
            Configuration::application("test.app", "fixture", "cache-run")
                .required_anchors(&["content"]),
            cx,
        )
        .unwrap();
    });
    let renders = std::rc::Rc::new(std::cell::Cell::new(0));
    let (handle, owner) = cx.update(|cx| {
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|cx| CachedOwner {
                child: cx.new(|_| CachedLeaf {
                    renders: renders.clone(),
                }),
            })
        })
        .unwrap()
    });
    let first = capture(cx, handle, None).unwrap();
    assert_eq!(first.nodes.len(), 3);
    let before = renders.get();
    let second = capture(cx, handle, None).unwrap();
    assert_eq!(second.nodes.len(), 3);
    assert!(
        renders.get() > before,
        "a request must not reuse last-frame geometry"
    );
    cx.update(|cx| owner.update(cx, |_, cx| cx.notify()));
    assert_eq!(capture(cx, handle, None).unwrap().nodes.len(), 3);
}

#[gpui_kit::test]
fn no_registered_root_is_not_a_component_fallback(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);
    let handle = cx.update(|cx| {
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| gpui_kit::Empty)
        })
        .unwrap()
        .0
    });
    cx.update_window(handle, |_, window, cx| {
        assert!(matches!(
            Session::for_window(window, cx),
            Err(Error::RootUnavailable)
        ))
    })
    .unwrap();
}

#[gpui_kit::test]
fn foreign_app_with_equal_local_window_ids_is_rejected(cx: &mut TestAppContext) {
    let mut other = cx.new_app();
    let (first, _view) = setup(cx);
    other.update(|cx| {
        gpui_kit::init(cx);
        register(
            Configuration::application("test.app", "fixture", "other-app-run")
                .required_anchors(&["content"]),
            cx,
        )
        .unwrap();
    });
    let second = other.update(|cx| {
        gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| {
            cx.new(|_| View {
                rows: vec![10, 20],
                extra: false,
                overlay: false,
            })
        })
        .unwrap()
        .0
    });
    assert_eq!(first, second, "exercise the equal per-App handle case");
    let session = cx
        .update_window(first, |_, window, cx| {
            let session = Session::for_window(window, cx).unwrap();
            session
                .request(
                    Request::capture("source", "fixture", Duration::from_secs(2), 1_048_576),
                    window,
                )
                .unwrap();
            window.render_frame(cx);
            session
        })
        .unwrap();
    cx.run_until_parked();
    let (capture, root_id) = cx
        .update_window(first, |_, window, _| {
            let capture = session.take(window).unwrap().unwrap();
            assert!(capture.to_json(&session, window).is_ok());
            (
                capture,
                window
                    .root::<gpui_kit::base::Root>()
                    .flatten()
                    .unwrap()
                    .entity_id(),
            )
        })
        .unwrap();
    other
        .update_window(second, |_, window, _| {
            assert_eq!(
                root_id,
                window
                    .root::<gpui_kit::base::Root>()
                    .flatten()
                    .unwrap()
                    .entity_id()
            );
            assert!(
                matches!(capture.to_json(&session, window), Err(Error::StaleTarget)),
                "foreign App window must not validate an original App capture"
            );
            assert!(matches!(
                session.request(
                    Request::capture("foreign", "fixture", Duration::from_secs(2), 1_048_576),
                    window
                ),
                Err(Error::StaleTarget)
            ));
        })
        .unwrap();
}
