pub(crate) use crate::frame::Sample;
use crate::{
    Configuration, Error, Request,
    frame::{FrameMetadata, Lease, Mounted, Pending, State},
};
use gpui_kit::{Bounds, Pixels, Window, base::Root};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};

static GENERATIONS: AtomicU64 = AtomicU64::new(0);
pub(crate) fn next_generation() -> Result<u64, Error> {
    GENERATIONS
        .try_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .map(|n| n + 1)
        .map_err(|_| Error::Limit)
}
/// Main-thread producer owned by exactly one app window/root.
#[derive(Clone)]
pub struct Session(pub(crate) Rc<RefCell<State>>);
/// Immutable observed layout; publication checks the full emitted occurrence set.
#[derive(Clone)]
pub struct Capture {
    pub(crate) owner: Weak<RefCell<State>>,
    pub(crate) config: Configuration,
    pub(crate) request: Request,
    pub(crate) instance: u64,
    pub(crate) sequence: u64,
    pub(crate) frame: u64,
    pub(crate) surface: u64,
    pub(crate) start: f64,
    pub(crate) end: f64,
    pub(crate) scale: f64,
    pub(crate) rem_size: f64,
    pub(crate) viewport: [f64; 2],
    pub(crate) window_id: String,
    pub(crate) samples: Vec<Sample>,
    pub(crate) bindings: Vec<Weak<Lease>>,
}
impl Session {
    pub fn new(config: Configuration, window: &Window) -> Result<Self, Error> {
        if config.window_root {
            return Err(Error::RootUnavailable);
        }
        Self::create(config, window)
    }
    pub(crate) fn create(config: Configuration, window: &Window) -> Result<Self, Error> {
        if !cfg!(debug_assertions) {
            return Err(Error::Disabled);
        }
        config.validate()?;
        Ok(Self(Rc::new(RefCell::new(State::new(
            config,
            window.window_handle(),
            Rc::downgrade(window.text_system()),
        )?))))
    }
    pub(crate) fn bind_root(&self, window: &Window) -> Result<(), Error> {
        let mut state = self.0.borrow_mut();
        if state.closed
            || state.window != window.window_handle()
            || !Weak::ptr_eq(&state.window_instance, &Rc::downgrade(window.text_system()))
        {
            return Err(Error::StaleTarget);
        }
        if state.config.window_root {
            let actual = window
                .root::<Root>()
                .flatten()
                .ok_or(Error::RootUnavailable)?
                .entity_id();
            if state.root_entity.is_some_and(|root| root != actual) {
                return Err(Error::StaleTarget);
            }
            state.root_entity = Some(actual);
        }
        Ok(())
    }
    fn seal(&self) {
        let mut state = self.0.borrow_mut();
        if state.config.window_root && !state.retired {
            return;
        }
        state.seal(Rc::downgrade(&self.0));
    }
    pub(crate) fn retire_window_frame(&self, frame: u64, window: &Window) {
        if self.bind_root(window).is_err() {
            self.close();
            return;
        }
        {
            let mut state = self.0.borrow_mut();
            if state.frame != frame || state.in_frame {
                return;
            }
            state.retired = true;
        }
        self.seal();
    }
    /// Arm one bounded request. Root requests have no selector by default.
    pub fn request(&self, request: Request, window: &mut Window) -> Result<(), Error> {
        self.bind_root(window)?;
        if self.0.borrow().in_frame {
            return Err(Error::Busy);
        }
        self.seal();
        let mut state = self.0.borrow_mut();
        if state.pending.is_some() || state.result.is_some() {
            return Err(Error::Busy);
        }
        request.validate(&state.config)?;
        let component = if let Some(id) = &request.component {
            let mut matches = state.current.iter().filter(|r| &r.id == id && r.alive());
            let first = matches.next().cloned().ok_or(Error::IncompleteScope)?;
            if matches.next().is_some() {
                return Err(Error::AmbiguousTarget);
            }
            Some(first)
        } else {
            None
        };
        state.pending = Some(Pending {
            request,
            started: Instant::now(),
            samples: Vec::new(),
            component,
            metadata: None,
        });
        window.refresh();
        Ok(())
    }
    /// Call after Window::draw has completed. Native callback recipe is in the example.
    pub fn take(&self, window: &Window) -> Option<Result<Capture, Error>> {
        if let Err(error) = self.bind_root(window) {
            return Some(Err(error));
        }
        if self.0.borrow().in_frame {
            return None;
        }
        self.seal();
        let mut state = self.0.borrow_mut();
        if state
            .pending
            .as_ref()
            .is_some_and(|p| p.started.elapsed() > p.request.deadline)
        {
            state.pending = None;
            return Some(Err(Error::Deadline));
        }
        state.result.take()
    }
    pub fn cancel(&self) {
        let mut state = self.0.borrow_mut();
        if state.pending.take().is_some() || state.result.is_some() {
            state.result = Some(Err(Error::Cancelled));
        }
    }
    pub fn close(&self) {
        let mut state = self.0.borrow_mut();
        state.closed = true;
        state.pending = None;
        state.result = Some(Err(Error::StaleTarget));
        state.current.clear();
        state.previous.clear();
        state.stack.clear();
        state.in_frame = false;
        state.ready = false;
    }
    pub(crate) fn begin(&self, window: &Window) -> bool {
        if let Err(error) = self.bind_root(window) {
            self.0.borrow_mut().frame_error = Some(error);
            return false;
        }
        self.seal();
        let mut state = self.0.borrow_mut();
        if state.pending.as_ref().is_some_and(|p| p.metadata.is_some()) {
            state.pending = None;
            state.result = Some(Err(Error::StaleTarget));
        }
        state.start_frame();
        true
    }
    pub(crate) fn begin_window(&self, bounds: Bounds<Pixels>, window: &Window) {
        if !self.begin(window) {
            return;
        }
        let mut state = self.0.borrow_mut();
        let record = Mounted {
            id: "window_root".into(),
            path: None,
            generation: state.root_generation,
            lease: None,
        };
        state.add(record, bounds);
    }
    pub(crate) fn finish(&self, window: &Window) {
        let mut state = self.0.borrow_mut();
        if !state.in_frame {
            return;
        }
        state.in_frame = false;
        state.ready = true;
        if state.pending.is_none() {
            return;
        }
        let scale = f64::from(window.scale_factor());
        let rem_size = f64::from(f32::from(window.rem_size()));
        let viewport = window.viewport_size();
        if !scale.is_finite() || scale <= 0. || !rem_size.is_finite() || rem_size <= 0. {
            state.frame_error = Some(Error::InvalidGeometry);
        }
        let metadata = FrameMetadata {
            end: state.started.elapsed().as_secs_f64(),
            scale,
            rem_size,
            viewport: [
                f32::from(viewport.width).into(),
                f32::from(viewport.height).into(),
            ],
            window_id: format!("{:?}", window.window_handle().window_id()),
        };
        if let Some(pending) = &mut state.pending {
            pending.metadata = Some(metadata);
        }
    }
}
impl Capture {
    /// A repeated structural label has no single mount; inspect its returned SourceKeys.
    pub fn mount(&self, id: &str) -> Option<u64> {
        let mut matches = self.samples.iter().filter(|s| s.id == id);
        let first = matches.next()?.mount;
        matches.next().is_none().then_some(first)
    }
    pub fn surface_generation(&self) -> u64 {
        self.surface
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn to_json(&self, session: &Session, window: &Window) -> Result<Vec<u8>, Error> {
        if !Weak::ptr_eq(&self.owner, &Rc::downgrade(&session.0)) {
            return Err(Error::StaleTarget);
        }
        session.bind_root(window)?;
        let state = session.0.borrow();
        if state.closed
            || state.in_frame
            || state.frame != self.frame
            || state.root_generation != self.surface
            || state.pending.is_some()
            || self
                .bindings
                .iter()
                .any(|binding| binding.upgrade().is_none())
            || self.samples.iter().any(|s| {
                !state
                    .current
                    .iter()
                    .any(|r| r.id == s.id && r.generation == s.mount && r.alive())
            })
        {
            return Err(Error::StaleTarget);
        }
        crate::encoding::encode(self)
    }
}
