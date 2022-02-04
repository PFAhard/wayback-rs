#[cfg(feature = "threads")]
pub(crate) mod threading;
#[cfg(feature = "threads")]
pub use threading::structs;

pub mod utils;

#[cfg(not(feature = "threads"))]
pub(crate) mod app;
#[cfg(not(feature = "threads"))]
pub mod structs;
