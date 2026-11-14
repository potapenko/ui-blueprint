//! The application-owned framework root, including same-window deferred layout.
use crate::{Configuration, Error, Session};
use gpui_kit::{
    AnyElement, App, Bounds, Context, Element, ElementId, Empty, Global, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Render, Window,
    base::{Root, RootPlugin},
};

struct Settings(Configuration);
impl Global for Settings {}

/// Register once after other Root plugins and before opening windows. No OS installation.
pub fn register(config: Configuration, cx: &mut App) -> Result<(), Error> {
    if !cfg!(debug_assertions) {
        return Err(Error::Disabled);
    }
    config.validate()?;
    if !config.window_root {
        return Err(Error::InvalidScope);
    }
    if cx.has_global::<Settings>() {
        return Err(Error::Duplicate);
    }
    cx.set_global(Settings(config));
    Root::register_plugin::<WindowProbe>(cx, |window, cx| WindowProbe {
        session: Session::create(cx.global::<Settings>().0.clone(), window),
    });
    Ok(())
}
struct WindowProbe {
    session: Result<Session, Error>,
}
impl Drop for WindowProbe {
    fn drop(&mut self) {
        if let Ok(session) = &self.session {
            session.close();
        }
    }
}
impl Render for WindowProbe {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}
impl RootPlugin for WindowProbe {
    fn decorate(
        &self,
        surface: AnyElement,
        _: &Root,
        _: &mut Window,
        _: &mut App,
    ) -> impl IntoElement {
        match &self.session {
            Ok(session) => WindowElement {
                inner: surface,
                session: session.clone(),
            }
            .into_any_element(),
            Err(_) => surface,
        }
    }
}
impl Session {
    /// Resolve the probe belonging to this actual framework root; never a component fallback.
    pub fn for_window(window: &Window, cx: &App) -> Result<Self, Error> {
        let root = window
            .root::<Root>()
            .flatten()
            .ok_or(Error::RootUnavailable)?;
        let probe = root
            .read(cx)
            .plugin::<WindowProbe>()
            .ok_or(Error::RootUnavailable)?;
        let session = probe.read(cx).session.clone()?;
        session.bind_root(window)?;
        Ok(session)
    }
}
struct WindowElement {
    inner: AnyElement,
    session: Session,
}
impl IntoElement for WindowElement {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for WindowElement {
    type RequestLayoutState = ();
    type PrepaintState = ();
    // Preserve the erased surface's identity stack, layout and native accessibility.
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.inner.request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.session.begin_window(bounds, window);
        self.inner.prepaint(window, cx);
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner.paint(window, cx);
        // All deferred prepaint is complete here in the pinned framework. take()
        // seals lifetimes only after Window::draw has retired the previous frame.
        self.session.finish(window);
        let frame = self.session.0.borrow().frame;
        let session = self.session.clone();
        window.defer(cx, move |window, _| {
            session.retire_window_frame(frame, window)
        });
    }
}
