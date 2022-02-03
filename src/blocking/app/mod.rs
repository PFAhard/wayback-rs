/*!
Blocking api for wayback-rs
*/
use super::structs::{Flow, IndColl, IntoFlag, IntoFlow, Otx, SubsFlag, WaybackRs, VT};
use crate::structs::Verbose;
use crate::utils::WaybackError;
use crate::utils::{timing_decorator, wrapper::result_unwrapper};
use anyhow::{Context, Result};
use reqwest::blocking::{Client, Response};
use std::{collections::HashSet, time::Duration};

//Limit for https://Otx.alienvault.com/
//I can’t set a limit higher than 500. Otx still gives out a limit of 500.
const LIMIT: u16 = 500;

impl WaybackRs {
    pub fn build_client() -> Client {
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
    pub fn unique_result_scan_domains(&mut self) -> HashSet<String> {
        self.list()
            .into_iter()
            .flat_map(|domain| self.unique_result_scan_domain(domain))
            .collect::<HashSet<String>>()
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
    //     self.list()
    //         .into_iter()
    //         .map(|domain| {
    //             self.set_domain(domain);
    //             self.scan_domain()
    //         })
    //         .flatten()
    //         .collect::<Vec<String>>()
    // }

    /// like [`scan_domain`] but return vector of unique sorted urls
    pub fn unique_result_scan_domain(&mut self, domain: String) -> HashSet<String> {
        self.scan_domain(domain)
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
    pub fn scan_domain(&mut self, domain: String) -> HashSet<String> {
        let flow: Flow = Flow::into_flow(
            self.client(),
            domain,
            self.subs_flag(),
            self.expensive(),
            self.vt_key(),
            self.batch().to_vec(),
            self.verbose(),
        );

        flow.into_iter().flat_map(|f| f()).collect()
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
        client: &Client,
        domain: &str,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        let sub_wild_card = subs_flag.select("*.", "");

        //let urls = Self::get_wayback_paginator(client, domain, subs_flag)?;
        //make paginator
        let mut response = timing_decorator(
            "request_wa",
            || -> Result<Response> {
                client
                    .get(format!(
                        "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original",
                        sub_wild_card, domain
                    ))
                    .send()
                    .context("Failed to make a request to CC, again")
            },
            crate::structs::Verbose::Timing,
        )?;

        dbg!(&response);

        let vec = timing_decorator(
            "read_copy_wa",
            || -> Result<Vec<u8>> {
                let mut tmp_buf = Vec::new();

                response
                    .copy_to(&mut tmp_buf)
                    .context("Failed to read Web archive json (can be empty)")?;
                Ok(tmp_buf)
            },
            Verbose::Timing,
        )?;
        let urls = timing_decorator(
            "read_text_wa",
            || -> Result<String> { Ok(String::from_utf8_lossy(&vec).to_string()) },
            Verbose::Timing,
        )?
        .lines()
        .map(ToString::to_string)
        .collect::<HashSet<String>>();

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
        client: &Client,
        domain: &str,
        subs_flag: SubsFlag,
    ) -> HashSet<String> {
        let urls = batch
            .iter_mut()
            .flat_map(|url| {
                let client = client.clone();
                result_unwrapper(Self::get_common_crawl_url(
                    &client,
                    domain,
                    subs_flag,
                    Some((*url).to_string()),
                ))
            })
            .collect::<HashSet<String>>();

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
        client: &Client,
        domain: &str,
        subs_flag: SubsFlag,
        url: Option<String>,
    ) -> Result<HashSet<String>> {
        let sub_wild_card = subs_flag.select("*.", "");
        let url = url
            .unwrap_or_else(|| "https://index.commoncrawl.org/CC-MAIN-2021-49-index".to_string());

        let response = timing_decorator(
            "request_cc",
            || -> Result<Response> {
                client
                    .get(format!(
                        "{}?url={}{}/*&output=text&fields=url",
                        url, sub_wild_card, domain
                    ))
                    .send()
                    .context("Failed to make a request to CC")
            },
            crate::structs::Verbose::Timing,
        )?;

        if response.status().as_u16() == 404 {
            return Err(WaybackError::CommonCrawlNoCaptures.into());
        }

        let urls = response
            .text()?
            .lines()
            .map(ToString::to_string)
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
    /// let urls = get_virus_total_url("test.com", "7b70aadb8a89a5704f547d468924a77d551d6de8db58d34495f28d494afa8e9f");
    ///
    /// ```
    pub fn get_virus_total_url(
        client: &Client,
        domain: &str,
        vt_key: Option<&str>,
    ) -> Result<HashSet<String>> {
        if vt_key.is_none() {
            return Err(WaybackError::VirusTotalKey.into());
        }

        let response = timing_decorator(
            "request_vt",
            || -> Result<Response> {
                client
                    .get(format!(
                        "https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={}",
                        vt_key.unwrap(),
                        domain
                    ))
                    .send()
                    .context("Failed to make a request to VT")
            },
            crate::structs::Verbose::Timing,
        )?;

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
    pub fn get_otx_alienvault_url(client: &Client, domain: &str) -> Result<HashSet<String>> {
        let mut urls = HashSet::new();
        let mut page: u16 = 1;
        let mut has_next = true;

        while has_next {
            let response = timing_decorator(
                "request_otx",
                || -> Result<Response> {
                    client
                        .get(format!(
                            "https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit={}&page={}",
                            domain, LIMIT, page
                        ))
                        .send()
                        .context("Failed to make a request to otx")
                },
                crate::structs::Verbose::Timing,
            )?;

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

    pub fn get_wayback_paginator(
        client: &Client,
        domain: &str,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        let sub_wild_card = subs_flag.select("*.", "");

        let mut page: u8 = 0;

        let page_response = timing_decorator(
            "0page_wa",
            || -> Result<Response> {
                client
            .get(format!(
                "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original&page={}",
                sub_wild_card, domain, page
            ))
            .send()
            .context("Failed to make a request to CC")
            },
            Verbose::Timing,
        )?;
        let max_page_header = page_response.headers().get("x-cdx-num-pages");
        dbg!(&max_page_header);
        let max_page = (max_page_header.unwrap().to_str()?).parse::<u8>()?;
        let mut urls = timing_decorator("read_lines", || -> Result<HashSet<String>> {
            Ok(page_response
                .text()?
                .lines()
                .map(ToString::to_string)
                .collect::<HashSet<String>>())
        }, Verbose::Timing)?;
        
        while page < max_page {
            page += 1;
            let page_response = timing_decorator(
                "nth_page_wa",
                || -> Result<Response> {
                    client
                .get(format!(
                    "https://web.archive.org/cdx/search/cdx?url={}{}/*&output=text&collapse=urlkey&fl=original&page={}",
                    sub_wild_card, domain, page
                ))
                .send()
                .context("Failed to make a request to CC")
                },
                Verbose::Timing,
            )?;
            urls.extend(
                timing_decorator("read_lines", || -> Result<HashSet<String>> {
                    Ok(page_response
                        .text()?
                        .lines()
                        .map(ToString::to_string)
                        .collect::<HashSet<String>>())
                }, Verbose::Timing)?
            );
        }

        Ok(urls)
    }
}
