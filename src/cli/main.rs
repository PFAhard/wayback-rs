#[cfg(feature = "cli")]
fn main() {
    #[cfg(all(feature = "async", feature = "threads"))]
    wayback::cli::concurrent::run_async_threads();
    #[cfg(all(feature = "async", not(feature = "threads")))]
    wayback::cli::concurrent::scan_domains_cli_async();
    #[cfg(all(not(feature = "async"), feature = "threads"))]
    wayback::cli::blocking::run_blocking_threads();
    #[cfg(not(any(feature = "async", feature = "threads")))]
    wayback::cli::blocking::scan_domains_cli();
}
