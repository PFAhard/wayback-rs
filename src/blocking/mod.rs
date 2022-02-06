#[cfg(feature = "threads")]
pub(crate) mod threading;
#[cfg(feature = "threads")]
pub(crate) use threading::structs;

pub(crate) mod utils;

#[cfg(not(feature = "threads"))]
pub(crate) mod app;
#[cfg(not(feature = "threads"))]
pub mod structs;
