/// Attempt to implement dns cache and file storage (maybe even merge)
use ureq::{Agent, AgentBuilder};

use super::{Expensive, SubsFlag, Verbose};
use crate::cli::args::Config;
use std::fmt::Display;

/// Struct for the main App
///
/// There are two diffetent approach of creating application
/// 1. With default trait:
/// ```
/// let mut wbs = WaybackRs::new();
/// ```
/// 2. With the ``Config``
/// ```
/// let config = Config::parse();
/// let mut wbs = WaybackRs::from_config(config);
/// ```
///
#[derive(Debug)]
pub struct WaybackRs {
    config: Config,
    batch: Vec<String>,
    client: Agent,
}

impl WaybackRs {
    /// Create application with predefined config
    pub fn from_config(config: Config) -> Self {
        let mut wbs = Self::new();
        wbs.config = config;
        wbs
    }

    /// Create application with default Trait
    /// # Default ``Config`` does not have any input
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn set_batch(&mut self, batch: Vec<String>) {
        self.batch = batch;
    }

    pub(crate) fn batch_cache(&self) -> bool {
        !self.batch.is_empty()
    }

    pub(crate) fn batch(&self) -> &[String] {
        self.batch.as_ref()
    }

    pub(crate) fn domain_is_some(&self) -> bool {
        self.config.domain_is_some()
    }

    /// Get a reference to the wayback rs's subs flag.
    pub(crate) fn subs_flag(&self) -> SubsFlag {
        self.config.subs_flag()
    }

    /// Get a unsafe reference to the wayback rs's vt key.
    pub(crate) fn vt_key(&self) -> Option<&str> {
        self.config.vt_key_ref()
    }

    /// Get a reference to the wayback rs's list.
    pub(crate) fn list(&self) -> Vec<String> {
        self.config.list()
    }

    /// Get a reference to the wayback rs's expensive.
    pub(crate) fn expensive(&self) -> Expensive {
        self.config.expensive()
    }

    pub(crate) fn output_is_none(&self) -> bool {
        self.config.output_is_none()
    }

    pub(crate) fn output(&self) -> Option<&String> {
        self.config.output()
    }

    pub(crate) fn verbose(&self) -> Verbose {
        self.config.verbose()
    }

    /// Set current domain for scan
    /// # If there was domain, it is dropped
    /// # ``WaybackRs::scan_domains`` will change current domain
    pub fn set_domain(&mut self, domain: String) {
        self.config.set_domain(domain);
    }

    /// Get a reference to the wayback rs's client.
    pub fn client(&self) -> &Agent {
        &self.client
    }

    /// Get a wrapped domain string ref
    pub fn domain(&self) -> Option<&str> {
        self.config.domain_ref()
    }
}

impl Default for WaybackRs {
    fn default() -> Self {
        let client = AgentBuilder::new().user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36").build();
        Self {
            config: Config::default(),
            batch: Vec::default(),
            client,
        }
    }
}

impl Display for WaybackRs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config)
    }
}
