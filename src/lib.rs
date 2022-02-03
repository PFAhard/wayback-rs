#![deny(
    unused_import_braces,
    unused_imports,
    bad_style,
    unused_must_use,
    dead_code,
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
    clippy::pedantic,
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
#![warn(clippy::missing_errors_doc, clippy::missing_panics_doc)]
// TO DO:
// Avoid duplicate of tail fn
// Avoid clonning the whole struct
// initiate batch on start of wbr
// Zero cost nth stream
// CC: filter {"message": "No Captures found for: {}/"}
// Otx: implement subs_flag on alientvault api?
// CC: batch: rewrite in favour of stream
// Otx: has next when error
// deser: all
// Avoid if true blocks
// Replace Common Crawl and Web archive to text
// fix async threads
// VT fields
// Slow down
// Web Archive Conection 111 [temp solution - blocking chunk]

/*!
Rust version of tomnomnom/waybackurls

Command line interface for fetching url from Wayback Machine, ``CommonCrawl``, ``VirusTotal``.

Install:

git clone <https://github.com/PFAhard/wayback-rs.git>
cd wayback-rs
cargo build --release
cp target/release/wayback ``path_to_bin_path``

Flag -e temporaly useless due to <https://groups.google.com/g/common-crawl/c/kEHzXZNu5To>
*/
pub mod cli;

#[cfg(not(feature = "async"))]
pub mod blocking;
#[cfg(feature = "async")]
pub mod concurrent;

#[cfg(not(feature = "async"))]
pub use blocking::{structs, utils};
#[cfg(feature = "async")]
pub use concurrent::{structs, utils};

#[test]
fn the_plastic_world_has_won() {
    let config = cli::args::Config::default();
    let mut wbs = structs::WaybackRs::from_config(config);
    let domain = "grob.ru".to_string();
    #[cfg(feature = "threads")]
    let mut wbs = std::sync::Arc::new(wbs);
    let urls = wbs.unique_result_scan_domain(domain);
    #[cfg(feature = "async")]
    let urls = tokio::runtime::Runtime::new().unwrap().block_on(urls);
    assert!(!urls.is_empty());
}
