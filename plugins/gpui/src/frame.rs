use crate::{
    Configuration, Error, Request,
    session::{Capture, next_generation},
};
use gpui_kit::{AnyWindowHandle, Bounds, EntityId, GlobalElementId, Pixels, WindowTextSystem};
use std::{any::TypeId, cell::RefCell, rc::Weak, time::Instant};

pub(crate) struct Lease {
    pub owner: Weak<RefCell<State>>,
    pub id: String,
    pub native_type: TypeId,
    pub generation: u64,
}
#[derive(Clone)]
pub(crate) struct Mounted {
    pub id: String,
    pub path: Option<GlobalElementId>,
    pub generation: u64,
    pub lease: Option<Weak<Lease>>,
}
impl Mounted {
    pub fn alive(&self) -> bool {
        self.lease
            .as_ref()
            .is_none_or(|lease| lease.upgrade().is_some())
    }
}
#[derive(Clone, Debug)]
pub(crate) struct Parent {
    pub id: String,
    pub mount: u64,
}
#[derive(Clone, Debug)]
pub(crate) struct Sample {
    pub id: String,
    pub parent: Option<Parent>,
    pub mount: u64,
    pub bounds: Bounds<Pixels>,
}
pub(crate) struct FrameMetadata {
    pub end: f64,
    pub scale: f64,
    pub rem_size: f64,
    pub viewport: [f64; 2],
    pub window_id: String,
}
pub(crate) struct Pending {
    pub request: Request,
    pub started: Instant,
    pub samples: Vec<Sample>,
    pub component: Option<Mounted>,
    pub metadata: Option<FrameMetadata>,
}
impl Pending {
    pub fn selects(&self, record: &Mounted) -> bool {
        if !self.request.root_request {
            return self.request.selected.contains(&record.id);
        }
        match &self.component {
            None => true,
            Some(component) => match (&component.path, &record.path) {
                (None, _) => true,
                (Some(parent), Some(child)) => child.starts_with(&parent[..]),
                _ => false,
            },
        }
    }
}
pub(crate) struct State {
    pub config: Configuration,
    pub window: AnyWindowHandle,
    // Opaque per-window allocation identity; no text-system content is read.
    pub window_instance: Weak<WindowTextSystem>,
    pub root_entity: Option<EntityId>,
    pub started: Instant,
    pub closed: bool,
    pub frame: u64,
    pub sequence: u64,
    pub instance: u64,
    pub root_generation: u64,
    pub current: Vec<Mounted>,
    pub previous: Vec<Mounted>,
    pub stack: Vec<usize>,
    pub frame_error: Option<Error>,
    pub in_frame: bool,
    pub ready: bool,
    pub retired: bool,
    pub pending: Option<Pending>,
    pub result: Option<Result<Capture, Error>>,
}
impl State {
    pub fn new(
        config: Configuration,
        window: AnyWindowHandle,
        window_instance: Weak<WindowTextSystem>,
    ) -> Result<Self, Error> {
        let root_generation = if config.window_root {
            next_generation()?
        } else {
            0
        };
        Ok(Self {
            config,
            window,
            window_instance,
            root_entity: None,
            started: Instant::now(),
            closed: false,
            frame: 0,
            sequence: 0,
            instance: next_generation()?,
            root_generation,
            current: Vec::new(),
            previous: Vec::new(),
            stack: Vec::new(),
            frame_error: None,
            in_frame: false,
            ready: false,
            retired: false,
            pending: None,
            result: None,
        })
    }
    pub fn start_frame(&mut self) {
        if self.in_frame {
            self.frame_error = Some(Error::Duplicate);
            return;
        }
        self.frame += 1;
        self.previous = std::mem::take(&mut self.current);
        self.current = Vec::with_capacity(self.previous.len().min(256));
        self.stack.clear();
        self.frame_error = None;
        self.in_frame = true;
        self.ready = false;
        self.retired = false;
    }
    pub fn add(&mut self, record: Mounted, bounds: Bounds<Pixels>) -> Option<usize> {
        if !self.in_frame {
            self.frame_error = Some(Error::InvalidScope);
            return None;
        }
        if self.current.len() == 256 {
            self.frame_error = Some(Error::Limit);
            return None;
        }
        if self
            .current
            .iter()
            .any(|r| r.path == record.path || (!self.config.window_root && r.id == record.id))
        {
            self.frame_error = Some(Error::Duplicate);
            return None;
        }
        let parent = self.stack.last().map(|ix| Parent {
            id: self.current[*ix].id.clone(),
            mount: self.current[*ix].generation,
        });
        if !self.config.window_root {
            let declared = self
                .config
                .markers
                .iter()
                .find(|m| m.id == record.id)
                .and_then(|m| m.parent.as_deref());
            if declared != parent.as_ref().map(|p| p.id.as_str()) {
                self.frame_error = Some(Error::InvalidScope);
            }
        }
        if self.pending.as_ref().is_some_and(|p| p.selects(&record)) {
            let values = [
                bounds.origin.x,
                bounds.origin.y,
                bounds.size.width,
                bounds.size.height,
            ]
            .map(f32::from);
            if values.iter().any(|v| !v.is_finite()) || values[2] < 0. || values[3] < 0. {
                self.frame_error = Some(Error::InvalidGeometry);
            } else if self
                .pending
                .as_ref()
                .is_some_and(|p| p.samples.len() >= 256)
            {
                self.frame_error = Some(Error::Limit);
            } else if let Some(pending) = &mut self.pending {
                pending.samples.push(Sample {
                    id: record.id.clone(),
                    parent,
                    mount: record.generation,
                    bounds,
                });
            }
        }
        let ix = self.current.len();
        self.current.push(record);
        Some(ix)
    }
    /// Called only outside the completed draw (take/request or next prepaint).
    /// Live previous leases without a new sample are cached replay, not fresh geometry.
    pub fn seal(&mut self, owner: Weak<RefCell<State>>) {
        if !self.ready {
            return;
        }
        self.ready = false;
        let missing: Vec<_> = self
            .previous
            .iter()
            .filter(|previous| {
                previous.alive()
                    && !self
                        .current
                        .iter()
                        .any(|current| current.generation == previous.generation)
            })
            .cloned()
            .collect();
        if self.current.len() + missing.len() > 256 {
            self.frame_error = Some(Error::Limit);
        }
        if let Some(mut pending) = self.pending.take() {
            let error = self.capture_error(&pending, &missing);
            if let Some(error) = error {
                self.result = Some(Err(error));
            } else if let Some(metadata) = pending.metadata.take() {
                if self.config.window_root {
                    self.resolve_parents(&mut pending.samples);
                }
                self.sequence += 1;
                let bindings = self
                    .current
                    .iter()
                    .filter(|r| pending.selects(r))
                    .filter_map(|r| r.lease.clone())
                    .collect();
                self.result = Some(Ok(Capture {
                    owner,
                    config: self.config.clone(),
                    request: pending.request,
                    instance: self.instance,
                    sequence: self.sequence,
                    frame: self.frame,
                    surface: self.root_generation,
                    start: pending.started.duration_since(self.started).as_secs_f64(),
                    end: metadata.end,
                    scale: metadata.scale,
                    rem_size: metadata.rem_size,
                    viewport: metadata.viewport,
                    window_id: metadata.window_id,
                    samples: pending.samples,
                    bindings,
                }));
            } else {
                self.result = Some(Err(Error::IncompleteScope));
            }
        }
        self.current.retain(Mounted::alive);
        // Keep only live metadata when an ordinary cached draw skipped callbacks.
        // Geometry from these records is NEVER copied into a new capture.
        if self.current.len() + missing.len() <= 256 {
            self.current.extend(missing);
        } else {
            self.current.clear();
            self.frame_error = Some(Error::Limit);
        }
        self.previous.clear();
    }
    fn capture_error(&self, pending: &Pending, missing: &[Mounted]) -> Option<Error> {
        if self.closed {
            return Some(Error::StaleTarget);
        }
        if let Some(error) = self.frame_error {
            return Some(error);
        }
        if pending.started.elapsed() > pending.request.deadline {
            return Some(Error::Deadline);
        }
        if pending
            .request
            .expected_surface
            .is_some_and(|x| x != self.root_generation)
        {
            return Some(Error::StaleTarget);
        }
        if missing.iter().any(|r| pending.selects(r)) {
            return Some(Error::IncompleteScope);
        }
        if self
            .current
            .iter()
            .any(|r| pending.selects(r) && !r.alive())
        {
            return Some(Error::StaleTarget);
        }
        if pending.component.as_ref().is_some_and(|component| {
            !self
                .current
                .iter()
                .any(|r| r.generation == component.generation && r.alive())
        }) {
            return Some(Error::StaleTarget);
        }
        if self
            .config
            .required
            .iter()
            .any(|id| !self.current.iter().any(|r| &r.id == id && r.alive()))
        {
            return Some(Error::IncompleteScope);
        }
        let expected = if pending.request.root_request {
            self.current.iter().filter(|r| pending.selects(r)).count()
        } else {
            pending.request.selected.len()
        };
        if pending.samples.len() != expected || pending.samples.is_empty() {
            return Some(Error::IncompleteScope);
        }
        if pending
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
            return Some(Error::StaleTarget);
        }
        None
    }
    fn resolve_parents(&self, samples: &mut [Sample]) {
        for sample in samples {
            let Some(record) = self.current.iter().find(|r| r.generation == sample.mount) else {
                continue;
            };
            let Some(path) = &record.path else {
                sample.parent = None;
                continue;
            };
            let parent = self
                .current
                .iter()
                .filter(|candidate| candidate.generation != record.generation)
                .filter(|candidate| {
                    candidate
                        .path
                        .as_ref()
                        .is_none_or(|p| p.len() < path.len() && path.starts_with(&p[..]))
                })
                .max_by_key(|candidate| candidate.path.as_ref().map_or(0, |p| p.len()));
            sample.parent = parent.map(|p| Parent {
                id: p.id.clone(),
                mount: p.generation,
            });
        }
    }
}
