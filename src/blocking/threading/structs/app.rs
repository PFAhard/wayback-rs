use std::sync::Arc;

use super::{Expensive, NetThreads, SubsFlag, Verbose};
use crate::cli::args::Config;

#[derive(Debug)]
pub struct WaybackRs {
    config: Config,
    batch: Vec<String>,
}

impl WaybackRs {
    pub(crate) fn from_config(config: Config) -> Self {
        Self::new(config)
    }

    pub(crate) fn new(config: Config) -> Self {
        Self {
            config,
            batch: vec![],
        }
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
    pub(crate) fn domain(&self) -> String {
        self.config.domain_unchecked()
    }

    pub(crate) fn domain_is_some(&self) -> bool {
        self.config.domain_is_some()
    }

    pub(crate) fn domain_arc(&self) -> Arc<String> {
        Arc::new(self.domain())
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
}
