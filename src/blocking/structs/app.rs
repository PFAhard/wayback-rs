use std::fmt::Display;

use reqwest::blocking::Client;
use crate::cli::args::Config;
use super::{Expensive, SubsFlag, Verbose};

#[derive(Debug)]
pub struct WaybackRs {
    config: Config,
    client: Client,
    batch: Vec<String>,
}

impl WaybackRs {
    pub fn from_config(config: Config) -> Self {
        Self::new(config)
    }

    fn new(config: Config) -> Self {
        Self {
            config,
            client: Self::build_client(),
            batch: vec![],
        }
    }

    #[cfg(not(feature = "threads"))]
    pub fn client(&self) -> Client {
        self.client.clone()
    }

    #[cfg(feature = "threads")]
    pub fn client(&self) -> std::sync::Arc<Client> {
        self.client.clone()
    }

    pub fn set_batch(&mut self, batch: Vec<String>) {
        self.batch = batch;
    }

    pub fn batch_cache(&self) -> bool {
        !self.batch.is_empty()
    }

    pub fn batch(&self) -> &[String] {
        self.batch.as_ref()
    }

    /// Get a unsafe reference to the wayback rs's domain.
    pub fn domain(&self) -> &str {
        self.config.domain_unchecked()
    }

    pub fn domain_is_some(&self) -> bool {
        self.config.domain_is_some()
    }

    /// Get a reference to the wayback rs's subs flag.
    pub fn subs_flag(&self) -> SubsFlag {
        self.config.subs_flag()
    }

    /// Get a unsafe reference to the wayback rs's vt key.
    pub fn vt_key(&self) -> Option<String> {
        self.config.vt_key()
    }

    /// Get a reference to the wayback rs's list.
    pub fn list(&self) -> Vec<String> {
        self.config.list()
    }

    /// Get a reference to the wayback rs's expensive.
    pub fn expensive(&self) -> Expensive {
        self.config.expensive()
    }

    #[cfg(feature = "threads")]
    pub fn net_threads(&self) -> bool {
        self.config.net_threads()
    }

    #[cfg(feature = "threads")]
    pub fn threads(&self) -> u8 {
        self.config.threads()
    }

    #[cfg(feature = "threads")]
    pub fn expensive_threads(&self) -> u8 {
        self.config.expensive_threads()
    }

    pub fn output_is_none(&self) -> bool {
        self.config.output_is_none()
    }

    pub fn output(&self) -> Option<&String> {
        self.config.output()
    }

    pub fn verbose(&self) -> Verbose {
        self.config.verbose()
    }

    // /// Get a mutable reference to the wayback rs's batch.
    // pub fn batch_mut(&mut self) -> &mut Vec<String> {
    //     &mut self.batch
    // }
}

impl Display for WaybackRs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.config)
    }
}