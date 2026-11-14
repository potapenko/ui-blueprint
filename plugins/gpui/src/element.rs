use crate::{
    Error, Session,
    frame::{Lease, Mounted},
    session::next_generation,
};
use gpui_kit::{
    A11ySubtreeBuilder, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, Pixels, Window, accesskit,
};
use std::{
    any::TypeId,
    cell::RefCell,
    rc::{Rc, Weak},
};

/// Geometry marker on an existing native-ID-bearing element; apply builders first.
pub struct ProbeElement<E> {
    inner: E,
    session: Session,
    id: String,
    legacy_root: bool,
}
struct RootLease {
    state: Weak<RefCell<crate::frame::State>>,
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
struct OccurrenceSlot(Rc<Lease>);
/// Opaque Element phase state; the original child's state is delegated unchanged.
pub struct PaintState<P> {
    inner: P,
    _root: Option<Rc<RootLease>>,
    _occurrence: Option<Rc<Lease>>,
    active: bool,
}
impl Session {
    pub fn mark<T: IntoElement>(
        &self,
        id: &str,
        element: T,
    ) -> Result<ProbeElement<T::Element>, Error> {
        crate::types::validate_id(id)?;
        let (window_root, legacy_root) = {
            let state = self.0.borrow();
            let ix = state.config.markers.iter().position(|m| m.id == id);
            if !state.config.window_root && ix.is_none() {
                return Err(Error::InvalidScope);
            }
            (
                state.config.window_root,
                !state.config.window_root && ix == Some(0),
            )
        };
        if window_root && id == "window_root" {
            return Err(Error::InvalidIdentity);
        }
        let inner = element.into_element();
        if inner.id().is_none() {
            return Err(Error::InvalidIdentity);
        }
        Ok(ProbeElement {
            inner,
            session: self.clone(),
            id: id.into(),
            legacy_root,
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
        if self.session.bind_root(window).is_err() {
            return PaintState {
                inner: self
                    .inner
                    .prepaint(id, inspector, bounds, layout, window, cx),
                _root: None,
                _occurrence: None,
                active: false,
            };
        }
        let Some(id) = id.filter(|id| id.len() <= 256) else {
            self.session.0.borrow_mut().frame_error = Some(Error::InvalidIdentity);
            return PaintState {
                inner: self
                    .inner
                    .prepaint(id, inspector, bounds, layout, window, cx),
                _root: None,
                _occurrence: None,
                active: false,
            };
        };
        let root = if self.legacy_root {
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
                            state.root_generation = next_generation().unwrap_or(0);
                            if state.root_generation == 0 {
                                state.closed = true;
                            }
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
        if self.legacy_root {
            self.session.begin(window);
        }
        if self.session.0.borrow().current.len() >= 256 {
            self.session.0.borrow_mut().frame_error = Some(Error::Limit);
            return PaintState {
                inner: self
                    .inner
                    .prepaint(Some(id), inspector, bounds, layout, window, cx),
                _root: root,
                _occurrence: None,
                active: true,
            };
        }
        let weak = Rc::downgrade(&self.session.0);
        let public_id = self.id.clone();
        let native_type = TypeId::of::<E>();
        let occurrence = window.with_element_state(id, |old: Option<OccurrenceSlot>, _| {
            let lease = old
                .map(|slot| slot.0)
                .filter(|lease| {
                    Weak::ptr_eq(&lease.owner, &weak)
                        && lease.id == public_id
                        && lease.native_type == native_type
                })
                .unwrap_or_else(|| {
                    Rc::new(Lease {
                        owner: weak,
                        id: public_id.clone(),
                        native_type,
                        generation: next_generation().unwrap_or(0),
                    })
                });
            (lease.clone(), OccurrenceSlot(lease))
        });
        let pushed = {
            let mut state = self.session.0.borrow_mut();
            if occurrence.generation == 0 {
                state.frame_error = Some(Error::Limit);
            }
            let record = Mounted {
                id: self.id.clone(),
                path: Some(id.clone()),
                generation: occurrence.generation,
                lease: Some(Rc::downgrade(&occurrence)),
            };
            let ix = state.add(record, bounds);
            if !state.config.window_root {
                if let Some(ix) = ix {
                    state.stack.push(ix);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        };
        let inner = self
            .inner
            .prepaint(Some(id), inspector, bounds, layout, window, cx);
        if pushed {
            self.session.0.borrow_mut().stack.pop();
        }
        PaintState {
            inner,
            _root: root,
            _occurrence: Some(occurrence),
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
        if self.legacy_root && paint.active {
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
