#![deny(
    unused_import_braces,
    unused_imports,
    bad_style,
    unused_must_use,
    //dead_code,
    unstable_name_collisions,
    unused_assignments,
    unreachable_patterns,
    missing_copy_implementations,
    missing_debug_implementations,
    //missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    //unreachable_pub,
    const_err,
    no_mangle_generic_items,
    overflowing_literals,
    private_in_public,
    while_true,
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    nonstandard_style,
    rust_2018_idioms
)]
#![deny(
    //clippy::pedantic,
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::cargo
)]
#![allow(
    clippy::multiple_crate_versions,
    clippy::must_use_candidate,
    clippy::module_name_repetitions
)]
#[cfg(feature = "nom_test")]
pub mod otx_nom;
#[cfg(feature = "serde_test")]
pub mod otx_serde;
pub mod otx_stream;
