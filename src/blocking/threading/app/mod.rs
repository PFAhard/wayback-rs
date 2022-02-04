/*!
Blocking api for wayback-rs
*/
use super::structs::{Flow, IndColl, IntoFlag, IntoFlow, NetThreads, Otx, SubsFlag, WaybackRs, VT};
use crate::utils::wrapper::result_unwrapper;
use crate::utils::WaybackError;
use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::slice::{ParallelSliceMut, ParallelSlice};
use std::collections::HashSet;
use std::sync::Arc;

//Limit for https://Otx.alienvault.com/
//I can’t set a limit higher than 50. Otx still gives out a limit of 50.
const LIMIT: u16 = 500;

impl WaybackRs {
    /// Scan all sources for list of domains and return vector of urls
    ///
    /// # Examples
    ///
    /// ```
    /// use wayback::scan_domain;
    ///
    /// let domains = vec!["test.com".to_string(),"sub.test.com".to_string(),"live.com".to_string(),"domain.com".to_string()]
    /// let urls = scan_domains(domains, true, None);
    ///
    /// ```
    pub(crate) fn scan_domains(&mut self) -> HashSet<String> {
        let chunk_size = self.list().len() / usize::from(self.threads());
        self.list()
            .par_chunks_mut(if chunk_size.eq(&0) { 1 } else { chunk_size })
            .flat_map_iter(|chunk| {
                chunk
                    .iter_mut()
                    .flat_map(|domain| self.scan_domain())
                    .collect::<HashSet<String>>()
            })
            .collect::<HashSet<String>>()
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
    pub(crate) fn scan_domain(&self) -> HashSet<String> {
        let flow: Flow = Flow::into_flow(
            self.domain_arc(),
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
    /// # Errors
    /// return anyhow Error over:
    /// 
    /// [`ureq::Error`]
    /// 
    /// [`std::io::Error`]
    pub fn get_wayback_url(
        domain: &Arc<String>,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        Self::get_wayback_paginator(domain, subs_flag)
        // let sub_wild_card = subs_flag.select("*.", "");

        // let url = format!(
        //     "https://web.archive.org/cdx/search/cdx?url={sub_wild_card}{domain}/*&output=text&collapse=urlkey&fl=original"
        // );

        // let mut buffer = Vec::new();
        // ureq::get(&url)
        //     .set("Host", "web.archive.org")
        //     .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
        //     .call()?
        //     .into_reader()
        //     .read_to_end(&mut buffer)?;
        
        // Ok(String::from_utf8_lossy(&buffer)
        // .lines()
        //     .map(ToString::to_string)
        //     .collect::<HashSet<String>>())
    }

    pub(crate) fn request_collection(&mut self) -> Result<()> {
        if self.batch_cache() {
            return Ok(());
        }

        self.set_batch(
            ureq::get("https://index.commoncrawl.org/collinfo.json")
                .set("Host", "index.commoncrawl.org")
                .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
                .call()?
                .into_json::<IndColl>()?.get()
            );

        Ok(())
    }

    ///
    ///
    /// request all indexes and make requests
    ///
    ///
    pub fn get_batch_common_crawl(
        mut batch: Vec<String>,
        domain: &Arc<String>,
        expensive_threads: u8,
        subs_flag: SubsFlag,
    ) -> HashSet<String> {
        let urls = batch
            .iter_mut()
            .flat_map(|url| {
                let domain = domain.clone();
                result_unwrapper(Self::get_common_crawl_url(
                    &domain,
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
    /// # Errors
    /// return anyhow Error over:
    /// ``WaybackError::CommonCrawlNoCaptures``
    /// [`ureq::Error`]
    ///
    /// [`std::io::Error`]
    pub fn get_common_crawl_url(
        domain: &Arc<String>,
        subs_flag: SubsFlag,
        url: Option<String>,
    ) -> Result<HashSet<String>> {
        let sub_wild_card = subs_flag.select("*.", "");
        let url = url
            .unwrap_or_else(|| "https://index.commoncrawl.org/CC-MAIN-2021-49-index".to_string());

        let url = format!(
            "{url}?url={sub_wild_card}{domain}/*&output=text&fields=url"
        );
        match ureq::get(&url)
                .set("Host", "index.commoncrawl.org")
                .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
                .call() {
                    Ok(response) => {
                        Ok(response
                            .into_string()?
                            .lines()
                            .map(ToString::to_string)
                            .collect::<HashSet<String>>())
                    },
                    Err(ureq::Error::Status(code, _)) => {
                        if code == 404 {
                            return Err(WaybackError::CommonCrawlNoCaptures.into());
                        } 
                        Err(WaybackError::UreqError.into())
                    },
                    Err(err) => Err(err.into())
                }
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
    /// # Errors
    /// return anyhow Error over:
    /// ``WaybackError::VirusTotalKey``
    /// ``WaybackError::VirusTotalNoContent``
    /// [`ureq::Error`]
    ///
    /// [`std::io::Error`]
    pub fn get_virus_total_url(
        domain: &Arc<String>,
        vt_key: Option<&str>,
    ) -> Result<HashSet<String>> {
        if vt_key.is_none() {
            return Err(WaybackError::VirusTotalKey.into());
        }
        let url = format!(
            "https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={domain}",
            vt_key.unwrap_or_else(|| unreachable!()),
        );
        match ureq::get(&url)
            .set("Host", "virustotal.com")
            .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
            .call() {
                Ok(response) => {
                    Ok(response.into_json::<VT>()?.consume())
                },
                Err(ureq::Error::Status(code, _)) => {
                    if code == 204 {
                        return Err(WaybackError::VirusTotalNoContent.into());
                    } 
                    Err(WaybackError::UreqError.into())
                },
                Err(err) => Err(err.into())
            }
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
    /// # Errors
    /// return anyhow Error over:
    ///
    /// [`ureq::Error`]
    ///
    /// [`std::io::Error`]
    pub fn get_otx_alienvault_url(domain: &Arc<String>) -> Result<HashSet<String>> {
        let mut urls = HashSet::new();
        let mut page: u16 = 1;
        let mut has_next = true;

        while has_next {
            let url = format!(
                "https://otx.alienvault.com/api/v1/indicators/domain/{domain}/url_list?limit={LIMIT}&page={page}"
            );
            match ureq::get(&url)
                .set("Host", "otx.alienvault.com")
                .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
                .call()?
                .into_json::<Otx>() {
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


    pub(crate) fn get_wayback_paginator(
        domain: &Arc<String>,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        #[inline]
        fn one_page(
            domain: &Arc<String>,
            subs_flag: SubsFlag,
            page: u64,
        ) -> Result<HashSet<String>> {
            let sub_wild_card = subs_flag.select("*.", "");

            let url = format!(
                "https://web.archive.org/cdx/search/cdx?url={sub_wild_card}{domain}/*&output=text&collapse=urlkey&fl=original&page={page}"
            );

            dbg!(page);

            Ok(ureq::get(&url)
                .set("Host", "web.archive.org")
                .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
                .call()?
                .into_string()?
                .lines()
                .map(ToString::to_string)
                .collect::<HashSet<String>>())  
        }

        #[inline]
        fn one_chunk(
            domain: &Arc<String>,
            subs_flag: SubsFlag,
            chunk: &[u64],
        ) -> HashSet<String> {
            chunk
                .iter()
                .flat_map(|page| 
                    result_unwrapper(one_page(domain, subs_flag, *page))
                    
                )
                .collect::<HashSet<String>>()
        }

        let sub_wild_card = subs_flag.select("*.", "");

        let url = format!(
            "https://web.archive.org/cdx/search/cdx?url={sub_wild_card}{domain}/*&output=text&collapse=urlkey&fl=original&page=0"
        );

        let response  = ureq::get(&url)
                .set("Host", "web.archive.org")
                .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
                .call()?;

        let max_page_header = response.header("x-cdx-num-pages").ok_or(WaybackError::WebArchivePaginator)?;
        let max_page = (max_page_header).parse::<u64>()?;

        let zero_url = response
            .into_string()?
            .lines()
            .map(ToString::to_string)
            .collect::<HashSet<String>>();

        Ok((1..max_page)
            .collect::<Vec<u64>>()
            .par_chunks(7)
            .flat_map_iter(|chunk| {
                one_chunk(domain, subs_flag, chunk)
            })
            .chain(zero_url)
            .collect::<HashSet<String>>())
    }
}
