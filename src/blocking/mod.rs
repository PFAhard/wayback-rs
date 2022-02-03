#[cfg(feature = "threads")]
pub mod threading;
#[cfg(feature = "threads")]
pub use threading::structs;

pub mod utils;

#[cfg(not(feature = "threads"))]
pub mod app;
#[cfg(not(feature = "threads"))]
pub mod structs;