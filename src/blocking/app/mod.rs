/*!
Blocking api for wayback-rs
*/
use super::structs::{Flow, IndColl, IntoFlag, IntoFlow, Otx, SubsFlag, WaybackRs, VT};
use crate::utils::wrapper::result_unwrapper;
use crate::utils::WaybackError;
use anyhow::Result;
use std::collections::HashSet;
use std::io::Read;
use std::rc::Rc;

//Limit for https://Otx.alienvault.com/
//I can’t set a limit higher than 500. Otx still gives out a limit of 500.
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
    pub fn scan_domains(&mut self) -> HashSet<String> {
        self.list()
            .into_iter()
            .flat_map(|domain| {
                self.set_domain(domain);
                self.scan_domain()
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
    pub fn scan_domain(&mut self) -> HashSet<String> {
        let flow: Flow = Flow::into_flow(
            self.domain_rc(),
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
    /// # Errors
    /// return anyhow Error over:
    /// 
    /// [`ureq::Error`]
    /// 
    /// [`std::io::Error`]
    pub fn get_wayback_url(
        domain: &Rc<String>,
        subs_flag: SubsFlag,
    ) -> Result<HashSet<String>> {
        let sub_wild_card = subs_flag.select("*.", "");

        let url = format!(
            "https://web.archive.org/cdx/search/cdx?url={sub_wild_card}{domain}/*&output=text&collapse=urlkey&fl=original"
        );

        let mut buffer = Vec::new();
        ureq::get(&url)
            .set("Host", "web.archive.org")
            .set("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36")
            .call()?
            .into_reader()
            .read_to_end(&mut buffer)?;
        
        Ok(String::from_utf8_lossy(&buffer)
        .lines()
            .map(ToString::to_string)
            .collect::<HashSet<String>>())
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
        
        domain: &Rc<String>,
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
        domain: &Rc<String>,
        subs_flag: SubsFlag,
        url: Option<String>,
    ) -> Result<HashSet<String>> {
        let sub_wild_card = subs_flag.select("*.", "");
        let url = url
            .unwrap_or_else(|| "https://index.commoncrawl.org/CC-MAIN-2021-49-index".to_string());

        let url = format!(
            "{}?url={}{}/*&output=text&fields=url",
            url, sub_wild_card, domain
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
        domain: &Rc<String>,
        vt_key: Option<&str>,
    ) -> Result<HashSet<String>> {
        if vt_key.is_none() {
            return Err(WaybackError::VirusTotalKey.into());
        }
        let url = format!(
            "https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={}",
            vt_key.unwrap_or_else(|| unreachable!()),
            domain
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
    pub fn get_otx_alienvault_url( 
        domain: &Rc<String>,
    ) -> Result<HashSet<String>> {
        let mut urls = HashSet::new();
        let mut page: u16 = 1;
        let mut has_next = true;

        while has_next {
            let url = format!(
                "https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit={}&page={}",
                domain, LIMIT, page
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
}
