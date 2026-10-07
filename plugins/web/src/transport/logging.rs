use std::sync::{
    OnceLock,
    atomic::{AtomicBool, Ordering},
};

static DELEGATE: OnceLock<&'static dyn log::Log> = OnceLock::new();
static INSTALLED: AtomicBool = AtomicBool::new(false);
static FILTER: Filter = Filter;
struct Filter;

fn upstream(target: &str) -> bool {
    target == "tungstenite" || target.starts_with("tungstenite::")
}
impl log::Log for Filter {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        !upstream(metadata.target()) && DELEGATE.get().is_some_and(|d| d.enabled(metadata))
    }
    fn log(&self, record: &log::Record<'_>) {
        // log! does not promise to call enabled first. Filter again before delegation.
        if !upstream(record.target())
            && let Some(delegate) = DELEGATE.get()
        {
            delegate.log(record);
        }
    }
    fn flush(&self) {
        if let Some(delegate) = DELEGATE.get() {
            delegate.flush();
        }
    }
}

/// Install once, before another logger. Only tungstenite targets are suppressed;
/// the host delegate retains all other logging policy. Does not change max_level.
/// A preinstalled logger is never replaced. Safe transport refuses until installed.
pub fn install_log_boundary(delegate: &'static dyn log::Log) -> Result<(), super::Failure> {
    if DELEGATE.set(delegate).is_err() || log::set_logger(&FILTER).is_err() {
        return Err(super::Failure::plain(super::ErrorKind::LoggingBoundary));
    }
    INSTALLED.store(true, Ordering::Release);
    Ok(())
}
pub(super) fn installed() -> bool {
    INSTALLED.load(Ordering::Acquire)
}
