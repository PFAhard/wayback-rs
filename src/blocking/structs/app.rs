use super::{Expensive, SubsFlag, Verbose};
use crate::cli::args::Config;
use std::{fmt::Display, rc::Rc};

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
#[derive(Debug, Default)]
pub struct WaybackRs {
    config: Config,
    batch: Vec<String>,
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

    /// Get a unsafe reference to the wayback rs's domain.
    pub(crate) fn domain_rc(&self) -> Rc<String> {
        Rc::new(self.config.domain_unchecked())
    }

    pub(crate) fn domain_is_some(&self) -> bool {
        self.config.domain_is_some()
    }

    /// Get a reference to the wayback rs's subs flag.
    pub(crate) fn subs_flag(&self) -> SubsFlag {
        self.config.subs_flag()
    }

    /// Get a unsafe reference to the wayback rs's vt key.
    pub(crate) fn vt_key(&self) -> Option<String> {
        self.config.vt_key()
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
}

impl Display for WaybackRs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config)
    }
}
