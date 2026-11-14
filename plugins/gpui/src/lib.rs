//! Explicit GPUI layout collection. The caller owns the window, requests and IO.
#![forbid(unsafe_code)]

mod element;
mod encoding;
mod session;
mod types;

pub use element::ProbeElement;
pub use session::{Capture, Session};
pub use types::{Configuration, Error, Marker, Request};
