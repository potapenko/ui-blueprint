//! Explicit GPUI layout collection. The caller owns the window, requests and IO.
#![forbid(unsafe_code)]

mod element;
mod encoding;
mod frame;
mod session;
mod types;
mod window;

pub use element::ProbeElement;
pub use session::{Capture, Session};
pub use types::{Configuration, Error, Marker, Request};
pub use window::register;
