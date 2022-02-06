use super::{Expensive, SubsFlag, Verbose};
use crate::cli::args::Config;
use reqwest::Client;

#[derive(Debug)]
pub struct WaybackRs {
    config: Config,
    client: Client,
    batch: Vec<String>,
}

impl WaybackRs {
    pub(crate) fn from_config(config: Config) -> Self {
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
    pub(crate) fn client(&self) -> Client {
        self.client.clone()
    }

    #[cfg(feature = "threads")]
    pub(crate) fn client(&self) -> Arc<Client> {
        self.client.clone()
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
        self.config.domain().unwrap()
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

    #[cfg(feature = "threads")]
    pub(crate) fn net_threads(&self) -> bool {
        self.config.net_threads()
    }

    #[cfg(feature = "threads")]
    pub(crate) fn threads(&self) -> u8 {
        self.config.threads()
    }

    #[cfg(feature = "threads")]
    pub(crate) fn expensive_threads(&self) -> u8 {
        self.config.expensive_threads()
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

    // /// Get a mutable reference to the wayback rs's batch.
    // pub(crate) fn batch_mut(&mut self) -> &mut Vec<String> {
    //     &mut self.batch
    // }
}
