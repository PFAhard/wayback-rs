//! Utils
pub(crate) mod errors;
pub(crate) mod from_file;
pub(crate) mod wrapper;

pub(crate) use errors::WaybackError;
pub(crate) use wrapper::{error_unwrapper, timing_decorator};
