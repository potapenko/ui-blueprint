use crate::{Configuration, Error, Request};
use gpui_kit::{AnyWindowHandle, Bounds, Pixels, Window};
use std::{
    any::TypeId,
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

#[derive(Clone, Debug)]
pub(crate) struct Sample {
    pub id: String,
    pub parent: Option<String>,
    pub mount: u64,
    pub bounds: Bounds<Pixels>,
}
#[derive(Default)]
pub(crate) struct Mounted {
    pub native_path: Option<gpui_kit::GlobalElementId>,
    pub native_type: Option<TypeId>,
    pub generation: u64,
    pub last_frame: u64,
}
pub(crate) struct Pending {
    pub request: Request,
    pub started: Instant,
    pub samples: Vec<Sample>,
}
pub(crate) struct State {
    pub config: Configuration,
    pub window: AnyWindowHandle,
    pub started: Instant,
    pub closed: bool,
    pub frame: u64,
    pub sequence: u64,
    pub instance: u64,
    pub root_generation: u64,
    pub mounts: Vec<Mounted>,
    pub stack: Vec<usize>,
    pub frame_error: Option<Error>,
    pub pending: Option<Pending>,
    pub result: Option<Result<Capture, Error>>,
}

/// Main-thread session for exactly one app window/root. Clone handles, not state.
#[derive(Clone)]
pub struct Session(pub(crate) Rc<RefCell<State>>);

/// One immutable layout observation. Publication validates against its owning session.
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
}

impl Session {
    pub fn new(config: Configuration, window: &Window) -> Result<Self, Error> {
        if !cfg!(debug_assertions) {
            return Err(Error::Disabled);
        }
        config.validate()?;
        let mounts = (0..config.markers.len())
            .map(|_| Mounted::default())
            .collect();
        Ok(Self(Rc::new(RefCell::new(State {
            config,
            window: window.window_handle(),
            started: Instant::now(),
            closed: false,
            frame: 0,
            sequence: 0,
            instance: next_generation()?,
            root_generation: 0,
            mounts,
            stack: Vec::new(),
            frame_error: None,
            pending: None,
            result: None,
        }))))
    }

    /// Arm one request and schedule a normal redraw. Does not focus or change app state.
    pub fn request(&self, request: Request, window: &mut Window) -> Result<(), Error> {
        let mut state = self.0.borrow_mut();
        if state.closed || state.window != window.window_handle() {
            return Err(Error::StaleTarget);
        }
        if state.pending.is_some() || state.result.is_some() {
            return Err(Error::Busy);
        }
        request.validate(&state.config)?;
        state.pending = Some(Pending {
            request,
            started: Instant::now(),
            samples: Vec::new(),
        });
        window.refresh();
        Ok(())
    }

    /// Read after the requested frame. No previous capture is returned as a fallback.
    pub fn take(&self, window: &Window) -> Option<Result<Capture, Error>> {
        let mut state = self.0.borrow_mut();
        if state.window != window.window_handle() {
            return Some(Err(Error::StaleTarget));
        }
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
    }

    pub(crate) fn finish(&self, window: &Window) {
        let mut state = self.0.borrow_mut();
        let Some(pending) = state.pending.take() else {
            return;
        };
        let error = state.frame_error.or_else(|| {
            if state.closed || state.window != window.window_handle() {
                Some(Error::StaleTarget)
            } else if pending.started.elapsed() > pending.request.deadline {
                Some(Error::Deadline)
            } else if pending
                .request
                .expected_surface
                .is_some_and(|x| x != state.root_generation)
            {
                Some(Error::StaleTarget)
            } else if pending.samples.len() != pending.request.selected.len() {
                Some(Error::IncompleteScope)
            } else if pending
                .request
                .expected_mounts
                .iter()
                .any(|(id, generation)| {
                    !pending
                        .samples
                        .iter()
                        .any(|s| &s.id == id && s.mount == *generation)
                })
            {
                Some(Error::StaleTarget)
            } else {
                None
            }
        });
        if let Some(error) = error {
            state.result = Some(Err(error));
            return;
        }
        let scale = f64::from(window.scale_factor());
        let rem_size = f64::from(f32::from(window.rem_size()));
        let viewport = window.viewport_size();
        if !scale.is_finite() || scale <= 0.0 || !rem_size.is_finite() || rem_size <= 0.0 {
            state.result = Some(Err(Error::InvalidGeometry));
            return;
        }
        state.sequence += 1;
        state.result = Some(Ok(Capture {
            owner: Rc::downgrade(&self.0),
            config: state.config.clone(),
            request: pending.request,
            instance: state.instance,
            sequence: state.sequence,
            frame: state.frame,
            surface: state.root_generation,
            start: pending.started.duration_since(state.started).as_secs_f64(),
            end: state.started.elapsed().as_secs_f64(),
            scale,
            rem_size,
            viewport: [
                f64::from(f32::from(viewport.width)),
                f64::from(f32::from(viewport.height)),
            ],
            window_id: format!("{:?}", window.window_handle().window_id()),
            samples: pending.samples,
        }));
    }
}

impl Capture {
    pub fn mount(&self, id: &str) -> Option<u64> {
        self.samples.iter().find(|s| s.id == id).map(|s| s.mount)
    }
    pub fn surface_generation(&self) -> u64 {
        self.surface
    }
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Validate current publication authority, then encode one bounded canonical document.
    pub fn to_json(&self, session: &Session, window: &Window) -> Result<Vec<u8>, Error> {
        if !Weak::ptr_eq(&self.owner, &Rc::downgrade(&session.0)) {
            return Err(Error::StaleTarget);
        }
        let state = session.0.borrow();
        if state.closed
            || state.window != window.window_handle()
            || state.frame != self.frame
            || state.root_generation != self.surface
            || state.pending.is_some()
            || self.samples.iter().any(|s| {
                let ix = state.config.markers.iter().position(|m| m.id == s.id);
                ix.is_none_or(|ix| {
                    state.mounts[ix].generation != s.mount
                        || state.mounts[ix].last_frame != self.frame
                })
            })
        {
            return Err(Error::StaleTarget);
        }
        crate::encoding::encode(self)
    }
}
