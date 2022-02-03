/*!
Blocking api for wayback-rs
*/
use super::structs::{Flow, IndColl, IntoFlag, IntoFlow, NetThreads, Otx, SubsFlag, WaybackRs, VT};
use crate::utils::wrapper::result_unwrapper;
use crate::utils::WaybackError;
use anyhow::{Context, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::ParallelSliceMut;
use reqwest::blocking::Client;
use std::sync::Arc;
use std::time::Duration;

//Limit for https://Otx.alienvault.com/
//I can’t set a limit higher than 50. Otx still gives out a limit of 50.
const LIMIT: u8 = 50;

impl WaybackRs {
    pub fn build_client() -> Arc<Client> {
        if let Ok(client) = Client::builder()
            .timeout(Duration::from_secs(300))
            .tcp_keepalive(Duration::from_secs(300))
            .tls_built_in_root_certs(true)
            .build()
        {
            Arc::new(client)
        } else {
            Arc::new(Client::new())
        }
    }

    /// like [`scan_domains`] but return vector of unique sorted urls
    pub fn unique_result_scan_domains(&self, mut list: Vec<String>) -> Vec<String> {
        let chunk_size = list.len() / usize::from(self.threads());
        let mut urls = list
            .par_chunks_mut(if chunk_size.eq(&0) { 1 } else { chunk_size })
            .flat_map_iter(|chunk| {
                chunk
                    .iter_mut()
                    .flat_map(|domain| self.unique_result_scan_domain((*domain).to_string()))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();

        urls.sort();
        urls.dedup();
        urls
    }

    // /// Scan all sources for list of domains and return vector of urls
    // ///
    // /// # Examples
    // ///
    // /// ```
    // /// use wayback::scan_domain;
    // ///
    // /// let domains = vec!["test.com".to_string(),"sub.test.com".to_string(),"live.com".to_string(),"domain.com".to_string()]
    // /// let urls = scan_domains(domains, true, None);
    // ///
    // /// ```
    // pub fn scan_domains(&mut self) -> Vec<String> {
    //     self.slice()
    //         .to_vec()
    //         .into_iter()
    //         .map(|domain| {
    //             self.set_domain(domain.to_string());
    //             self.scan_domain()
    //         })
    //         .flatten()
    //         .collect::<Vec<String>>()
    // }

    /// like [`scan_domain`] but return vector of unique sorted urls
    pub fn unique_result_scan_domain(&self, domain: String) -> Vec<String> {
        let mut urls = self.scan_domain(domain);
        urls.sort();
        urls.dedup();
        urls
    }

    /// Scan all sources for one domain and return vector of urls
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::scan_domain;
    ///
    /// let urls = scan_domain("test.com", true, &None);
    ///
    /// ```
    pub fn scan_domain(&self, domain: String) -> Vec<String> {
        let flow: Flow = Flow::into_flow(
            self.client(),
            Arc::new(domain),
            self.subs_flag(),
            self.expensive(),
            self.vt_key(),
            self.batch().to_vec(),
            self.expensive_threads(),
            self.verbose(),
        );

        match self.disable_subthreads() {
            NetThreads::Enable => flow.into_par_iter().flat_map_iter(move |f| f()).collect(),
            NetThreads::Disable => flow.into_iter().flat_map(|f| f()).collect(),
        }
    }

    /// Makes a request to web.archive.org api and parses the result, returns a vector of urls or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::get_wayback_url;
    ///
    /// let urls = get_wayback_url("test.com", true);
    ///
    /// ```
    pub fn get_wayback_url(
        client: &Arc<Client>,
        domain: &Arc<String>,
        subs_flag: SubsFlag,
    ) -> Result<Vec<String>> {
        let sub_wild_card = subs_flag.select("*.", "");

        let response = client
            .get(format!(
                "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original",
                sub_wild_card, domain
            ))
            .send()
            .context("Failed to make a request to CC, again")?;

        let urls = response
            .text()
            .context("Failed to read Web archive json (can be empty)")?
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();

        Ok(urls)
    }

    pub fn request_collection(&mut self) -> Result<()> {
        if self.batch_cache() {
            return Ok(());
        }
        let response = self
            .client()
            .get("https://index.commoncrawl.org/collinfo.json")
            .send()
            .context("Failed to make a request to CC")?;
        let indcoll = response.json::<IndColl>()?.get();
        self.set_batch(indcoll);
        Ok(())
    }

    ///
    ///
    /// request all indexes and make requests
    ///
    ///
    pub fn get_batch_common_crawl(
        mut batch: Vec<String>,
        client: &Arc<Client>,
        domain: &Arc<String>,
        expensive_threads: u8,
        subs_flag: SubsFlag,
    ) -> Vec<String> {
        let chunk_size = batch.len() / usize::from(expensive_threads);
        let mut urls = batch
            .par_chunks_mut(if chunk_size.eq(&0) { 1 } else { chunk_size })
            .flat_map_iter(|chunk| {
                chunk
                    .iter_mut()
                    .flat_map(|url| {
                        let client = client.clone();
                        let domain = domain.clone();
                        result_unwrapper(Self::get_common_crawl_url(
                            &client,
                            &domain,
                            subs_flag,
                            Some((*url).to_string()),
                        ))
                    })
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<String>>();

        urls.sort();
        urls.dedup();

        urls
    }

    /// Makes a request to commoncrawl.org api and parses the result, returns a vector of urls or None.
    ///
    /// TO-DO enumerate index
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::get_common_crawl_url;
    ///
    /// let urls = get_common_crawl_url("test.com", true);
    ///
    /// ```
    pub fn get_common_crawl_url(
        client: &Arc<Client>,
        domain: &Arc<String>,
        subs_flag: SubsFlag,
        url: Option<String>,
    ) -> Result<Vec<String>> {
        let sub_wild_card = subs_flag.select("*.", "");
        let url = url
            .unwrap_or_else(|| "https://index.commoncrawl.org/CC-MAIN-2021-49-index".to_string());
        let response = client
            .get(format!(
                "{}?url={}{}/*&output=text&fields=url",
                url, sub_wild_card, domain
            ))
            .send()
            .context("Failed to make a request to CC")?;

        if response.status().as_u16() == 404 {
            return Err(WaybackError::CommonCrawlNoCaptures.into());
        }

        let urls = response
            .text()
            .unwrap()
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();

        Ok(urls)
    }

    /// Makes a request to virustotal.com api and parses the result, returns a vector of urls or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::get_virus_total_url;
    ///
    /// let urls = get_virus_total_url("test.com", "7b70aadb8a89a5704f547d468924a77d551d6de8db58d34495f28d494afa8e9f");
    ///
    /// ```
    pub fn get_virus_total_url(
        client: &Arc<Client>,
        domain: &Arc<String>,
        vt_key: Option<&str>,
    ) -> Result<Vec<String>> {
        if vt_key.is_none() {
            return Err(WaybackError::VirusTotalKey.into());
        }

        let response = client
            .get(format!(
                "https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={}",
                vt_key.unwrap(),
                domain
            ))
            .send()
            .context("Failed to make a request to VT")?;

        if response.status().as_u16() == 204 {
            return Err(WaybackError::VirusTotalNoContent.into());
        }

        let urls = response.json::<VT>().context("Failed to read VT text")?;

        Ok(urls.consume())
    }

    /// Makes a request to otx.alienvault.com api and parses the result, returns a vector of urls or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::get_otx_alienvault_url;
    ///
    /// let urls = get_otx_alienvault_url("test.com");
    ///
    /// ```
    pub fn get_otx_alienvault_url(
        client: &Arc<Client>,
        domain: &Arc<String>,
    ) -> Result<Vec<String>> {
        let mut urls = Vec::new();
        let mut page: u16 = 1;
        let mut has_next = true;

        while has_next {
            let response = client
                .get(format!(
                    "https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit={}&page={}",
                    domain, LIMIT, page
                ))
                .send()
                .context("Failed to make a request to otx")?;

            match response
                .json::<Otx>()
                .context("Failed to decode Otx from json")
            {
                Ok(otx) => {
                    has_next = otx.has_next();
                    page += 1;
                    urls.extend(otx.url_list());
                }
                Err(err) => {
                    has_next = false;
                    eprintln!("{}", err);
                }
            };
        }

        Ok(urls)
    }
}
