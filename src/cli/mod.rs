#![doc(hidden)]
pub mod args;

#[cfg(not(feature = "async"))]
pub mod blocking {
    /// Cli module for wayback-rs
    #[cfg(not(feature = "threads"))]
    pub fn scan_domains_cli() {
        use crate::{
            cli::args::Config,
            structs::{app_trace, IntoFlag, WaybackRs},
            utils::wrapper::result_unwrapper,
        };
        use clap::Parser;
        use std::{fs::File, io::Write};

        let config = Config::parse();
        let mut wbs = WaybackRs::from_config(config);

        app_trace(wbs.verbose());

        wbs.expensive()
            .restrict(|| result_unwrapper(wbs.request_collection()));

        let urls = if wbs.domain_is_some() {
            let domain = wbs.domain().to_string();
            wbs.unique_result_scan_domain(domain)
        } else {
            wbs.unique_result_scan_domains()
        };

        if wbs.output_is_none() {
            let mut out = std::io::stdout();
            for line in urls {
                writeln!(out, "{line}").unwrap();
            }
        } else {
            let mut out = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(wbs.output().unwrap())
                .unwrap();
            for line in urls {
                writeln!(out, "{line}").unwrap();
            }
        }
    }

    #[cfg(feature = "threads")]
    pub fn run_blocking_threads() {
        use super::args::Config;
        use crate::blocking::{
            structs::{app_trace, IntoFlag, WaybackRs},
            utils::wrapper::result_unwrapper,
        };
        use clap::Parser;
        use std::{fs::File, io::Write};

        let config = Config::parse();
        let mut wbs = WaybackRs::from_config(config);

        app_trace(wbs.verbose());

        wbs.expensive()
            .restrict(|| result_unwrapper(wbs.request_collection()));

        let urls = if wbs.domain_is_some() {
            let domain = wbs.domain().to_string();
            wbs.unique_result_scan_domain(domain)
        } else {
            let list = wbs.list();
            wbs.unique_result_scan_domains(list)
        };

        if wbs.output_is_none() {
            let mut out = std::io::stdout();
            for line in urls {
                writeln!(out, "{}", line)
                    .is_err()
                    .then(|| eprintln!("Error in writeln"));
            }
        } else {
            let mut out = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(wbs.output().unwrap())
                .unwrap();
            for line in urls {
                writeln!(out, "{}", line)
                    .is_err()
                    .then(|| eprintln!("Error in writeln"));
            }
        }
    }
}

#[cfg(feature = "async")]
pub mod concurrent {
    /// Async Cli module for wayback-rs
    #[cfg(not(feature = "threads"))]
    pub fn scan_domains_cli_async() {
        use std::{fs::File, io::Write};

        use crate::{
            cli::args::Config,
            concurrent::structs::app_trace,
            structs::{IntoFlag, WaybackRs},
            utils::error_unwrapper,
        };
        use clap::Parser;
        use tokio::runtime;

        let config = Config::parse();
        let mut wbs = WaybackRs::from_config(config);

        app_trace(wbs.verbose());

        let rt = runtime::Builder::new_current_thread()
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        let urls = rt.block_on(async {
            wbs.expensive()
                .restrict(error_unwrapper(wbs.request_collection()))
                .await;
            if wbs.domain_is_some() {
                let domain = wbs.domain().to_string();
                wbs.unique_result_scan_domain(domain).await
            } else {
                wbs.unique_result_scan_domains().await
            }
        });
        if wbs.output_is_none() {
            let mut out = std::io::stdout();
            for line in urls {
                writeln!(out, "{}", line)
                    .is_err()
                    .then(|| eprintln!("Error in writeln"));
            }
        } else {
            let mut out = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(wbs.output().unwrap())
                .unwrap();
            for line in urls {
                writeln!(out, "{}", line)
                    .is_err()
                    .then(|| eprintln!("Error in writeln"));
            }
        }
    }

    #[cfg(feature = "threads")]
    pub fn run_async_threads() {
        use std::{fs::File, io::Write, sync::Arc};

        use crate::{
            cli::args::Config,
            concurrent::structs::app_trace,
            structs::{IntoFlag, WaybackRs},
            utils::error_unwrapper,
        };
        use clap::Parser;
        use tokio::runtime;

        let config = Config::parse();
        let mut wbs = WaybackRs::from_config(config);

        app_trace(wbs.verbose());

        let rt: tokio::runtime::Runtime = runtime::Builder::new_multi_thread()
            .worker_threads(wbs.threads().into())
            .enable_io()
            .enable_time()
            .build()
            .unwrap();

        let (urls, wbs) = rt.block_on(async {
            wbs.expensive()
                .restrict(error_unwrapper(wbs.request_collection()))
                .await;
            let arc_wbs = Arc::new(wbs);
            let wbs = arc_wbs.clone();
            let urls = if arc_wbs.domain_is_some() {
                let domain = arc_wbs.domain().to_string();
                arc_wbs.unique_result_scan_domain(domain).await
            } else {
                arc_wbs.unique_result_scan_domains().await
            };
            (urls, wbs)
        });
        if wbs.output_is_none() {
            let mut out = std::io::stdout();
            for line in urls {
                writeln!(out, "{}", line)
                    .is_err()
                    .then(|| eprintln!("Error in writeln"));
            }
        } else {
            let mut out = File::options()
                .create(true)
                .write(true)
                .truncate(true)
                .open(wbs.output().unwrap())
                .unwrap();
            for line in urls {
                writeln!(out, "{}", line)
                    .is_err()
                    .then(|| eprintln!("Error in writeln"));
            }
        }
    }
}
