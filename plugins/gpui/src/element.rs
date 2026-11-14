use crate::{Error, Session, session::Sample};
use gpui_kit::{
    A11ySubtreeBuilder, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Window, accesskit,
};
use std::{
    any::TypeId,
    cell::RefCell,
    rc::{Rc, Weak},
};

/// Layout-transparent marker. Apply after the original element's builder methods.
pub struct ProbeElement<E> {
    inner: E,
    session: Session,
    ix: usize,
}

struct RootLease {
    state: Weak<RefCell<crate::session::State>>,
    generation: u64,
}
impl Drop for RootLease {
    fn drop(&mut self) {
        if let Some(state) = self.state.upgrade() {
            let mut state = state.borrow_mut();
            if state.root_generation == self.generation {
                state.closed = true;
                state.pending = None;
                state.result = Some(Err(Error::StaleTarget));
            }
        }
    }
}

/// Public only because Element exposes its associated state; fields stay private.
pub struct PaintState<P> {
    inner: P,
    _root: Option<Rc<RootLease>>,
    active: bool,
}

impl Session {
    /// Mark an existing element. Every marker needs an existing stable native ElementId.
    pub fn mark<T: IntoElement>(
        &self,
        id: &str,
        element: T,
    ) -> Result<ProbeElement<T::Element>, Error> {
        let state = self.0.borrow();
        let ix = state
            .config
            .markers
            .iter()
            .position(|m| m.id == id)
            .ok_or(Error::InvalidScope)?;
        let inner = element.into_element();
        if inner.id().is_none() {
            return Err(Error::InvalidIdentity);
        }
        Ok(ProbeElement {
            inner,
            session: self.clone(),
            ix,
        })
    }
}

impl<E: Element> IntoElement for ProbeElement<E> {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}

impl<E: Element> Element for ProbeElement<E> {
    type RequestLayoutState = E::RequestLayoutState;
    type PrepaintState = PaintState<E::PrepaintState>;
    fn id(&self) -> Option<ElementId> {
        self.inner.id()
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.inner.source_location()
    }
    fn request_layout(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        self.inner.request_layout(id, inspector, window, cx)
    }
    fn prepaint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        if self.session.0.borrow().closed {
            return PaintState {
                inner: self
                    .inner
                    .prepaint(id, inspector, bounds, layout, window, cx),
                _root: None,
                active: false,
            };
        }
        let root = if self.ix == 0 {
            // mark() requires an existing root ID; delegation preserves it exactly.
            let id = id.expect("validated root has its original ElementId");
            let weak = Rc::downgrade(&self.session.0);
            Some(
                window.with_element_state(id, |old: Option<Rc<RootLease>>, _| {
                    let lease = old
                        .filter(|lease| Weak::ptr_eq(&lease.state, &weak))
                        .unwrap_or_else(|| {
                            let mut state = self.session.0.borrow_mut();
                            if state.root_generation != 0 {
                                state.closed = true;
                            }
                            state.root_generation = match crate::session::next_generation() {
                                Ok(generation) => generation,
                                Err(error) => {
                                    state.closed = true;
                                    state.frame_error = Some(error);
                                    0
                                }
                            };
                            Rc::new(RootLease {
                                state: weak,
                                generation: state.root_generation,
                            })
                        });
                    (lease.clone(), lease)
                }),
            )
        } else {
            None
        };
        let pushed = {
            let mut state = self.session.0.borrow_mut();
            if self.ix == 0 {
                state.frame += 1;
                state.frame_error = None;
                state.stack.clear();
            }
            let declared_parent = state.config.markers[self.ix].parent.as_ref();
            let actual_parent = state.stack.last().map(|ix| &state.config.markers[*ix].id);
            if declared_parent != actual_parent || state.frame == 0 {
                state.frame_error = Some(Error::InvalidScope);
            }
            let frame = state.frame;
            if state.mounts[self.ix].last_frame == frame {
                state.frame_error = Some(Error::Duplicate);
            }
            let native_path = id.filter(|id| id.len() <= 256);
            if native_path.is_none() {
                state.frame_error = Some(Error::InvalidIdentity);
            }
            let changed = state.mounts[self.ix].native_path.as_ref() != native_path
                || state.mounts[self.ix].native_type != Some(TypeId::of::<E>());
            if state.mounts[self.ix].generation == 0
                || state.mounts[self.ix].last_frame + 1 < frame
                || changed
            {
                match crate::session::next_generation() {
                    Ok(generation) => state.mounts[self.ix].generation = generation,
                    Err(error) => state.frame_error = Some(error),
                }
            }
            state.mounts[self.ix].native_path = native_path.cloned();
            state.mounts[self.ix].native_type = Some(TypeId::of::<E>());
            state.mounts[self.ix].last_frame = frame;
            let marker = &state.config.markers[self.ix];
            let selected = state
                .pending
                .as_ref()
                .is_some_and(|p| p.request.selected.contains(&marker.id));
            if selected {
                let values = [
                    bounds.origin.x,
                    bounds.origin.y,
                    bounds.size.width,
                    bounds.size.height,
                ]
                .map(f32::from);
                if values.iter().any(|v| !v.is_finite()) || values[2] < 0.0 || values[3] < 0.0 {
                    state.frame_error = Some(Error::InvalidGeometry);
                } else {
                    let sample = Sample {
                        id: marker.id.clone(),
                        parent: marker.parent.clone(),
                        mount: state.mounts[self.ix].generation,
                        bounds,
                    };
                    if let Some(pending) = state.pending.as_mut() {
                        if pending.samples.len() < 256 {
                            pending.samples.push(sample);
                        } else {
                            state.frame_error = Some(Error::Limit);
                        }
                    }
                }
            }
            if state.stack.len() < 256 {
                state.stack.push(self.ix);
                true
            } else {
                state.frame_error = Some(Error::Limit);
                false
            }
        };
        let inner = self
            .inner
            .prepaint(id, inspector, bounds, layout, window, cx);
        if pushed {
            self.session.0.borrow_mut().stack.pop();
        }
        PaintState {
            inner,
            _root: root,
            active: true,
        }
    }
    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        paint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.inner
            .paint(id, inspector, bounds, layout, &mut paint.inner, window, cx);
        if self.ix == 0 && paint.active {
            self.session.finish(window);
        }
    }
    fn a11y_role(&self) -> Option<accesskit::Role> {
        self.inner.a11y_role()
    }
    fn write_a11y_info(&self, node: &mut accesskit::Node) {
        self.inner.write_a11y_info(node);
    }
    fn a11y_synthetic_children(
        &mut self,
        paint: &mut Self::PrepaintState,
        builder: &mut A11ySubtreeBuilder,
    ) {
        self.inner
            .a11y_synthetic_children(&mut paint.inner, builder);
    }
}
