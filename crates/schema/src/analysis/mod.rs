//! Local analysis format 0.2, reusing immutable core 0.1 source records.
//! Contract validation is separate from the engine's arithmetic verification.
mod codec;
mod results;
mod types;
mod validation;

pub use codec::{json_schema, validate_input_document};
pub use results::{validate_geometry_check_case, validate_measurement_case};
pub use types::*;
pub use validation::{validate_bound_evaluation, validate_evaluation_input, validate_query};
