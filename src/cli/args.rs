use std::fmt::Display;

use clap::Parser;

use crate::{
    structs::{Expensive, NetThreads, SubsFlag, Verbose},
    utils::from_file::FromFile,
};

#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct Config {
    #[clap(
        short = 'd',
        long,
        help = r#"use for scan one domain"#,
        conflicts_with("list")
    )]
    domain: Option<String>,
    #[clap(
        short,
        long,
        help = r#"file with domains for scan"#,
        conflicts_with("domain")
    )]
    list: Option<String>,
    #[clap(short, long, help = r#"scan for subdomain also"#, parse(from_flag))]
    subs_flag: SubsFlag,
    #[clap(short = 'A', long, help = r#"virus total api key"#)]
    vt_key: Option<String>,
    #[clap(
        short,
        long,
        help = r#"The number of threads used to run in parallel for different targets
App never use exact threads number: total_threads={'-t'...('-t'*('2 if -n' + '-E'))}
default behavior can be changed by the flags: --disable-subthreads or -E or -n"#,
        default_value = "1"
    )]
    threads: u8,
    #[clap(
        short,
        long,
        help = r#"Use More source than dafault take more time give much more results"#,
        parse(from_flag)
    )]
    expensive: Expensive,
    #[clap(
        short = 'D',
        long,
        help = r#"Use to disable subthreads behavior, it costs time"#,
        parse(from_flag)
    )]
    disable_subthreads: NetThreads,
    #[clap(
        short = 'E',
        long,
        help = r#"Number of Common Crawl expensive Scan (81 requests currently)"#,
        default_value = "4"
    )]
    expensive_threads: u8,
    #[clap(
        short,
        long,
        help = r#"v - timings
vv - warn
vvv - info
vvvv - debug
vvvvv - trace"#,
        parse(from_occurrences = Verbose::from),
    )]
    verbose: Verbose,
    #[clap(short = 'o', long, help = r#"Output file (preferable)"#)]
    output: Option<String>,
}

impl Config {
    /// Get domain.
    /// Panic if None
    pub(crate) fn domain_unchecked(&self) -> String {
        self.domain.clone().unwrap()
    }

    pub(crate) fn domain_is_some(&self) -> bool {
        self.domain.is_some()
    }

    /// Get a unsafe reference to the args's list.
    pub(crate) fn list(&self) -> Vec<String> {
        Vec::from_file(self.list.as_ref().unwrap())
    }

    // pub(crate) fn copy_list_path(&self) -> Option<String> {
    //     self.list.clone()
    // }

    /// Get a reference to the args's subs flag.
    pub(crate) fn subs_flag(&self) -> SubsFlag {
        self.subs_flag
    }

    /// Get a reference to the args's vt key.
    pub(crate) fn vt_key(&self) -> Option<String> {
        self.vt_key.clone()
    }

    // pub(crate) fn vt_key_unchecked(&self) -> String {
    //     match &self.vt_key {
    //         Some(k) => k.clone(),
    //         None => unreachable!("unchecked domain unwrap failed"),
    //     }
    // }

    /// Get a reference to the args's threads.
    #[cfg(feature = "threads")]
    pub(crate) fn threads(&self) -> u8 {
        self.threads
    }

    /// Get a reference to the args's expensive.
    pub(crate) fn expensive(&self) -> Expensive {
        self.expensive
    }

    // /// Get a reference to the config's expensive threads.
    // #[cfg(feature = "threads")]
    // pub(crate) fn expensive_threads(&self) -> u8 {
    //     self.expensive_threads
    // }

    /// Get a reference to the config's output.
    pub(crate) fn output(&self) -> Option<&String> {
        self.output.as_ref()
    }

    pub(crate) fn output_is_none(&self) -> bool {
        self.output.is_none()
    }

    // /// Get a reference to the config's disable subthreads.
    // pub(crate) fn disable_subthreads(&self) -> NetThreads {
    //     self.disable_subthreads
    // }

    /// Get a reference to the config's verbose.
    pub(crate) fn verbose(&self) -> Verbose {
        self.verbose
    }

    #[cfg(feature = "threads")]
    /// Get a reference to the config's expensive threads.
    pub(crate) fn expensive_threads(&self) -> u8 {
        self.expensive_threads
    }

    #[cfg(feature = "threads")]
    #[warn(dead_code)]
    /// Get a reference to the config's disable subthreads.
    pub(crate) fn disable_subthreads(&self) -> NetThreads {
        self.disable_subthreads
    }

    /// Set the config's domain.
    pub fn set_domain(&mut self, domain: String) {
        self.domain = Some(domain);
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            domain: None,
            list: None,
            subs_flag: SubsFlag::Disable,
            vt_key: None,
            threads: 1,
            expensive: Expensive::Disable,
            disable_subthreads: NetThreads::Disable,
            expensive_threads: 0,
            verbose: Verbose::None,
            output: None,
        }
    }
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
