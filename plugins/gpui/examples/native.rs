#[path = "support/window_view.rs"]
mod support;
use gpui_kit::{prelude::*, *};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let Some(directory) = args.get(1) else {
        eprintln!("usage: native SYSTEM_TEMP_OUTPUT [off]");
        return;
    };
    let directory = std::path::PathBuf::from(directory);
    let temp = std::env::temp_dir()
        .canonicalize()
        .expect("system temp exists");
    let directory = directory
        .canonicalize()
        .expect("caller creates output directory");
    assert!(
        directory.starts_with(temp),
        "outputs must be under system temp"
    );
    let build = std::env::var("UIB_EXAMPLE_BUILD")
        .expect("caller supplies exact binary SHA-256 in UIB_EXAMPLE_BUILD");
    let enabled = args.get(2).is_none_or(|value| value != "off");
    let run = format!(
        "run-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("UTC after epoch")
            .as_nanos()
    );
    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx| {
            gpui_kit::init(cx);
            if enabled {
                uiblueprint_gpui::register(
                    uiblueprint_gpui::Configuration::application(
                        "org.uiblueprint.gpui-example",
                        &build,
                        &run,
                    )
                    .required_anchors(&[
                        "content",
                        "toolbar",
                        "workspace",
                        "rows",
                        "input_frame",
                    ]),
                    cx,
                )
                .expect("valid Debug app attribution before window creation");
            }
            let (handle, view) = gpui_kit::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds::new(
                        point(px(140.), px(100.)),
                        size(px(720.), px(640.)),
                    ))),
                    titlebar: Some(TitlebarOptions {
                        title: Some("UI Blueprint GPUI probe".into()),
                        ..Default::default()
                    }),
                    app_id: Some("org.uiblueprint.gpui-example".into()),
                    ..Default::default()
                },
                cx,
                |window, cx| cx.new(|cx| support::WindowView::new(window, cx, enabled, directory)),
            )
            .expect("open diagnostic window");
            println!(
                "ready pid={} mode={}",
                std::process::id(),
                if enabled { "on" } else { "off" }
            );
            let background = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                loop {
                    let line = background
                        .spawn(async move {
                            let mut line = String::new();
                            let count = std::io::stdin().read_line(&mut line).unwrap_or(0);
                            (count, line)
                        })
                        .await;
                    if line.0 == 0 {
                        break;
                    }
                    if handle
                        .update(cx, |_, window, cx| {
                            view.update(cx, |view, cx| view.command(&line.1, window, cx))
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            })
            .detach();
        });
}
