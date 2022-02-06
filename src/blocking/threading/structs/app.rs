use std::{fmt::Display, sync::Arc};

use ureq::{Agent, AgentBuilder};

use super::{Expensive, NetThreads, SubsFlag, Verbose};
use crate::cli::args::Config;

#[derive(Debug)]
pub struct WaybackRs {
    config: Config,
    batch: Vec<String>,
    client: Arc<Agent>,
}

impl WaybackRs {
    pub(crate) fn from_config(config: Config) -> Self {
        let mut wbs = Self::new();
        wbs.config = config;
        wbs
    }

    pub(crate) fn new() -> Self {
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
    pub(crate) fn domain(&self) -> Option<String> {
        self.config.domain()
    }

    pub(crate) fn domain_is_some(&self) -> bool {
        self.config.domain_is_some()
    }

    pub(crate) fn domain_arc(&self) -> Option<Arc<String>> {
        self.domain().map(Arc::new)
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

    pub(crate) fn threads(&self) -> u8 {
        self.config.threads()
    }

    pub(crate) fn expensive_threads(&self) -> u8 {
        self.config.expensive_threads()
    }

    pub(crate) fn output_is_none(&self) -> bool {
        self.config.output_is_none()
    }

    pub(crate) fn output(&self) -> Option<&String> {
        self.config.output()
    }

    pub(crate) fn disable_subthreads(&self) -> NetThreads {
        self.config.disable_subthreads()
    }

    pub(crate) fn verbose(&self) -> Verbose {
        self.config.verbose()
    }

    /// Get a reference to the wayback rs's client.
    pub fn client(&self) -> Arc<Agent> {
        self.client.clone()
    }

    pub fn set_domain(&mut self, domain: String) {
        self.config.set_domain(domain);
    }
}

impl Default for WaybackRs {
    fn default() -> Self {
        let client = Arc::new(AgentBuilder::new().user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36").build());
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
