use std::{fmt, time::Duration};

/// A public semantic marker declaration, independent of its current mount.
#[derive(Clone, Debug)]
pub struct Marker {
    pub(crate) id: String,
    pub(crate) parent: Option<String>,
}

impl Marker {
    pub fn new(id: &str, parent: Option<&str>) -> Self {
        Self {
            id: id.into(),
            parent: parent.map(str::to_owned),
        }
    }
}

/// Caller-owned public attribution. `run` must be unique for this app launch.
#[derive(Clone, Debug)]
pub struct Configuration {
    pub(crate) app: String,
    pub(crate) build: String,
    pub(crate) run: String,
    pub(crate) scope: String,
    pub(crate) markers: Vec<Marker>,
}

impl Configuration {
    pub fn new(app: &str, build: &str, run: &str, scope: &str, markers: Vec<Marker>) -> Self {
        Self {
            app: app.into(),
            build: build.into(),
            run: run.into(),
            scope: scope.into(),
            markers,
        }
    }

    pub(crate) fn validate(&self) -> Result<(), Error> {
        for value in [&self.app, &self.build, &self.run, &self.scope] {
            validate_id(value)?;
        }
        if self.markers.is_empty() || self.markers.len() > 256 {
            return Err(Error::Limit);
        }
        for (ix, marker) in self.markers.iter().enumerate() {
            validate_id(&marker.id)?;
            if self.markers[..ix].iter().any(|m| m.id == marker.id) {
                return Err(Error::Duplicate);
            }
            // Ordered declarations make parent closure/cycle validation bounded and unambiguous.
            match marker.parent.as_deref() {
                None if ix == 0 => (),
                Some(parent) if self.markers[..ix].iter().any(|m| m.id == parent) => (),
                _ => return Err(Error::InvalidScope),
            }
        }
        Ok(())
    }
}

/// One bounded observation, with optional expected identities from a prior capture.
#[derive(Clone, Debug)]
pub struct Request {
    pub(crate) id: String,
    pub(crate) state: String,
    pub(crate) selected: Vec<String>,
    pub(crate) expected_mounts: Vec<(String, u64)>,
    pub(crate) expected_surface: Option<u64>,
    pub(crate) deadline: Duration,
    pub(crate) max_bytes: usize,
}

impl Request {
    pub fn new(
        id: &str,
        state: &str,
        selected: &[&str],
        deadline: Duration,
        max_bytes: usize,
    ) -> Self {
        Self {
            id: id.into(),
            state: state.into(),
            selected: selected.iter().map(|s| (*s).into()).collect(),
            expected_mounts: Vec::new(),
            expected_surface: None,
            deadline,
            max_bytes,
        }
    }
    pub fn with_expected_mount(mut self, marker: &str, generation: u64) -> Self {
        self.expected_mounts.push((marker.into(), generation));
        self
    }
    pub fn with_expected_surface(mut self, generation: u64) -> Self {
        self.expected_surface = Some(generation);
        self
    }
    pub(crate) fn validate(&self, config: &Configuration) -> Result<(), Error> {
        validate_id(&self.id)?;
        validate_id(&self.state)?;
        if self.selected.is_empty()
            || self.selected.len() > 256
            || self.max_bytes == 0
            || self.max_bytes > 1_048_576
            || self.deadline < Duration::from_millis(1)
            || self.deadline > Duration::from_secs(10)
            || self.expected_mounts.len() > 256
        {
            return Err(Error::Limit);
        }
        for (ix, id) in self.selected.iter().enumerate() {
            if self.selected[..ix].contains(id) {
                return Err(Error::Duplicate);
            }
            let marker = config
                .markers
                .iter()
                .find(|m| &m.id == id)
                .ok_or(Error::InvalidScope)?;
            if marker
                .parent
                .as_ref()
                .is_some_and(|p| !self.selected.contains(p))
            {
                return Err(Error::InvalidScope);
            }
        }
        if !self.selected.contains(&config.markers[0].id) {
            return Err(Error::InvalidScope);
        }
        for (ix, (id, generation)) in self.expected_mounts.iter().enumerate() {
            if *generation == 0
                || !self.selected.contains(id)
                || self.expected_mounts[..ix]
                    .iter()
                    .any(|(previous, _)| previous == id)
            {
                return Err(Error::InvalidScope);
            }
        }
        Ok(())
    }
}

pub(crate) fn validate_id(value: &str) -> Result<(), Error> {
    if value.is_empty()
        || value.len() > 64
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
    {
        return Err(Error::InvalidIdentity);
    }
    Ok(())
}

/// Payload-free failures; never include application text, paths or credentials.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    Disabled,
    InvalidIdentity,
    InvalidScope,
    Duplicate,
    IncompleteScope,
    StaleTarget,
    Busy,
    Cancelled,
    Deadline,
    Limit,
    InvalidGeometry,
    Encoding,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
