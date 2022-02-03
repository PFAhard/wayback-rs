/*!
Async api for wayback-rs
*/
use std::collections::HashSet;

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
const LIMIT: u16 = 500;

impl WaybackRs {
    pub(crate) fn build_client() -> Client {
        if let Ok(client) = Client::builder()
            .timeout(Duration::from_secs(300))
            .tcp_keepalive(Duration::from_secs(300))
            .tls_built_in_root_certs(true)
            .build()
        {
            client
        } else {
            Client::new()
        }
    }

    /// like [`scan_domains`] but return vector of unique sorted urls
    pub(crate) async fn unique_result_scan_domains(&mut self) -> HashSet<String> {
        let stream = futures::stream::iter(self.list());

        let urls = join_all(
            stream
                .fold(vec![], |mut acc, domain| async {
                    acc.push(self.scan_domain(domain));
                    acc
                })
                .await,
        )
        .await;

        urls.into_iter().flatten().collect::<HashSet<String>>()
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
    // pub  async fn scan_domains(&mut self) -> HashSet<String> {
    //     let stream = futures::stream::iter(self.list());

    //     let urls = join_all(stream.fold(vec![], |mut acc, domain| async {
    //         acc.push(self.scan_domain(domain, self.expensive()));
    //         acc
    //     }).await).await;

    //     urls.into_iter().flatten().collect()
    // }

    /// like [`scan_domain`] but return vector of unique sorted urls
    pub(crate) async fn unique_result_scan_domain(&self, domain: String) -> HashSet<String> {
        self.scan_domain(domain).await
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
    pub(crate) async fn scan_domain(&self, domain: String) -> HashSet<String> {
        let flow: Flow = Flow::into_flow(
            self.client(),
            domain,
            self.subs_flag(),
            self.expensive(),
            self.vt_key(),
            HashSet::from_iter(self.batch().to_vec()),
            self.verbose(),
        );
        let urls = join_all(flow).await;
        urls.into_iter().flatten().collect::<HashSet<String>>()
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
    pub(crate) async fn get_wayback_url(
        client: Client,
        domain: String,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        // let sub_wild_card = subs_flag.select("*.", "");

        let urls = Self::get_wayback_paginator(&client, &domain, subs_flag).await?;

        // let response = client
        //     .get(format!(
        //         "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original",
        //         sub_wild_card, domain
        //     ))
        //     .send()
        //     .await
        //     .context("Failed to make a request to CC, again")?;

        // let urls = response
        //     .text()
        //     .await
        //     .context("Failed to read Web archive json (can be empty)")?
        //     .lines()
        //     .map(std::string::ToString::to_string)
        //     .collect::<HashSet<String>>();

        Ok(urls)
    }

    pub(crate) async fn request_collection(&mut self) -> Result<()> {
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
    pub(crate) async fn get_batch_common_crawl(
        batch: HashSet<String>,
        client: Client,
        domain: String,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
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

        let urls = urls
            .into_iter()
            .flat_map(result_unwrapper)
            .collect::<HashSet<String>>();

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
    pub(crate) async fn get_common_crawl_url(
        client: Client,
        domain: String,
        subs_flag: SubsFlag,
        url: Option<String>,
    ) -> Result<HashSet<String>> {
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
            .collect::<HashSet<String>>();

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
    pub(crate) async fn get_virus_total_url(
        client: Client,
        domain: String,
        vt_key: Option<String>,
    ) -> Result<HashSet<String>> {
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
    pub(crate) async fn get_otx_alienvault_url(
        client: Client,
        domain: String,
    ) -> Result<HashSet<String>> {
        #[inline]
        async fn one_page(client: &Client, domain: &str, page: u64) -> Result<HashSet<String>> {
            Ok(
                client
                    .get(format!(
                        "https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit={}&page={}",
                        domain, LIMIT, page
                    ))
                    .send().await?
                    .json::<Otx>()
                    .await?
                    .url_list()
            )
        }

        let page_response = client
            .get(format!(
                "https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit={}&page=1",
                domain, LIMIT
            ))
            .send()
            .await?
            .json::<Otx>()
            .await?;
        let max_page = page_response.size() / u64::from(LIMIT) + 1;
        let zero_url = page_response.url_list();

        let range = futures::stream::iter(2..max_page);
        let flat_map = range
            .map(|page| one_page(&client, &domain, page))
            .collect::<Vec<_>>()
            .await;
        let join = join_all(flat_map).await;

        let urls = join
            .into_iter()
            .flat_map(result_unwrapper)
            .chain(zero_url)
            .collect::<HashSet<String>>();

        Ok(urls)
    }

    pub(crate) async fn get_wayback_paginator(
        client: &Client,
        domain: &str,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        #[inline]
        async fn one_page(
            client: &Client,
            domain: &str,
            subs_flag: SubsFlag,
            page: u64,
        ) -> Result<HashSet<String>> {
            let sub_wild_card = subs_flag.select("*.", "");

            let res = client.get(format!(
                "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original&page={}",
                sub_wild_card, domain, page
            ))
                .send()
                .await?
                .text()
                .await?
                .lines()
                .map(ToString::to_string)
                .collect::<HashSet<String>>();

            Ok(res)
        }

        #[inline]
        async fn one_chunk(
            client: &Client,
            domain: &str,
            subs_flag: SubsFlag,
            chunk: &[u64],
        ) -> Result<HashSet<String>> {
            let range = futures::stream::iter(chunk);
            let flat_map = range
                .map(|page| one_page(client, domain, subs_flag, *page))
                .collect::<Vec<_>>()
                .await;
            let join: Vec<Result<HashSet<String>>> = join_all(flat_map).await;

            let urls = join
                .into_iter()
                .flat_map(result_unwrapper)
                .collect::<HashSet<String>>();

            Ok(urls)
        }

        let sub_wild_card = subs_flag.select("*.", "");

        let page_response = client
                .get(format!(
                    "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original&page=0",
                    sub_wild_card, domain
                ))
                .send().await?;
        let max_page_header = page_response.headers().get("x-cdx-num-pages");
        let max_page = (max_page_header.unwrap().to_str()?).parse::<u64>()?;
        let zero_url = page_response
            .text()
            .await?
            .lines()
            .map(ToString::to_string)
            .collect::<HashSet<String>>();

        let range = 1..max_page;

        let mut urls = HashSet::new();
        for chunk in range.collect::<Vec<u64>>().chunks(15) {
            urls.extend(one_chunk(client, domain, subs_flag, chunk).await?);
        }
        urls.extend(zero_url);

        Ok(urls)
    }
}
