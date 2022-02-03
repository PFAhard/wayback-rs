/*!
Async api for wayback-rs
*/
use std::sync::Arc;

use anyhow::{Context, Result};
use futures::{future::join_all, StreamExt};
use reqwest::Client;
use tokio::time::Duration;

use crate::{
    structs::{IndColl, IntoFlag, Otx, WaybackRs, VT},
    utils::{result_unwrapper, WaybackError},
};

use super::structs::{Flow, IntoFlow, SubsFlag};

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
    pub async fn unique_result_scan_domains(self: Arc<Self>) -> Vec<String> {
        let list = self.list();

        let urls = join_all(
            list.into_iter()
                .map(|domain| tokio::spawn(self.clone().unique_result_scan_domain(domain))),
        )
        .await;

        let mut urls = urls
            .into_iter()
            .flat_map(std::result::Result::unwrap)
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
    // pub  async fn scan_domains(&mut self) -> Vec<String> {
    //     let stream = futures::stream::iter(self.list());

    //     let urls = join_all(stream.fold(vec![], |mut acc, domain| async {
    //         acc.push(self.scan_domain(domain, self.expensive()));
    //         acc
    //     }).await).await;

    //     urls.into_iter().flatten().collect()
    // }

    /// like [`scan_domain`] but return vector of unique sorted urls
    pub async fn unique_result_scan_domain(self: Arc<Self>, domain: String) -> Vec<String> {
        let mut urls = self.scan_domain(domain).await;
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
    pub async fn scan_domain(&self, domain: String) -> Vec<String> {
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
        let urls = join_all(flow.into_iter().map(tokio::spawn)).await;
        urls.into_iter()
            .flat_map(std::result::Result::unwrap)
            .collect::<Vec<String>>()
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
    pub async fn get_wayback_url(
        client: Arc<Client>,
        domain: Arc<String>,
        subs_flag: SubsFlag,
    ) -> Result<Vec<String>> {
        let sub_wild_card = subs_flag.select("*.", "");

        let response = client
            .get(format!(
                "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original",
                sub_wild_card, domain
            ))
            .send()
            .await
            .context("Failed to make a request to CC, again")?;

        let urls = response
            .text()
            .await
            .context("Failed to read Web archive json (can be empty)")?
            .lines()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>();

        Ok(urls)
    }

    pub async fn request_collection(&mut self) -> Result<()> {
        if self.batch_cache() {
            return Ok(());
        }
        let response = self
            .client()
            .get("https://index.commoncrawl.org/collinfo.json")
            .send()
            .await
            .context("Failed to make a request to CC")?;
        let indcoll = response.json::<IndColl>().await?.get();
        self.set_batch(indcoll);
        Ok(())
    }

    ///
    ///
    /// request all indexes and make requests
    ///
    ///
    pub async fn get_batch_common_crawl(
        batch: Vec<String>,
        client: Arc<Client>,
        domain: Arc<String>,
        subs_flag: SubsFlag,
    ) -> Result<Vec<String>> {
        let batch = futures::stream::iter(batch);
        let urls = join_all(
            batch
                .fold(vec![], |mut acc, url| async {
                    acc.push(Self::get_common_crawl_url(
                        client.clone(),
                        domain.clone(),
                        subs_flag,
                        Some(url),
                    ));
                    acc
                })
                .await,
        )
        .await;

        let mut urls = urls
            .into_iter()
            .flat_map(result_unwrapper)
            .collect::<Vec<String>>();
        urls.sort();
        urls.dedup();

        Ok(urls)
    }

    /// Makes a request to commoncrawl.org api and parses the result, returns a vector of urls or None.
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::get_common_crawl_url;
    ///
    /// let urls = get_common_crawl_url("test.com", true);
    ///
    /// ```
    pub async fn get_common_crawl_url(
        client: Arc<Client>,
        domain: Arc<String>,
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
            .await
            .context("Failed to make a request to CC")?;

        if response.status().as_u16() == 404 {
            return Err(WaybackError::CommonCrawlNoCaptures.into());
        }

        let urls = response
            .text()
            .await
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
    /// let urls = get_virus_total_url("test.com", "7bdb8a8992344a57043d58d7d46895f22ad4f2efa8e90aa544a77d551d6de8df");
    ///
    /// ```
    pub async fn get_virus_total_url(
        client: Arc<Client>,
        domain: Arc<String>,
        vt_key: Option<String>,
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
            .await
            .context("Failed to make a request to VT")?;

        if response.status().as_u16() == 204 {
            return Err(WaybackError::VirusTotalNoContent.into());
        }

        let urls = response
            .json::<VT>()
            .await
            .context("Failed to read VT text")?;

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
    pub async fn get_otx_alienvault_url(
        client: Arc<Client>,
        domain: Arc<String>,
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
                .await
                .context("Failed to make a request to otx")?;

            match response
                .json::<Otx>()
                .await
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
