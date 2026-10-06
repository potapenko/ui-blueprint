//! Model-free engineering export. Inputs are explicit; embedded references are never opened.
#![forbid(unsafe_code)]
mod compile;
mod observed;
mod package;
mod proposal;
mod types;
mod validate;
pub use compile::compile;
pub use package::Package;
pub use types::*;

/// Payload-free errors: neither Debug nor Display includes input strings or paths.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportError {
    InputLimit,
    OutputLimit,
    InvalidInput,
    InvalidSource,
    InvalidReference,
    InvalidGeometry,
    InvalidChain,
    IncompatibleViews,
    PrivateContent,
    UnsupportedVerification,
    DestinationExists,
    Io,
}
impl std::fmt::Display for ExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ExportError {}
type Result<T> = std::result::Result<T, ExportError>;
