use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub(crate) enum WaybackError {
    #[error("VirusTotal key unprovided")]
    VirusTotalKey,
    #[error("VirusTotal return 204 No Content")]
    VirusTotalNoContent,
    #[error("Common Crawl: No Captures found for url")]
    CommonCrawlNoCaptures,
    #[error("Request error")]
    UreqError,
    #[error("Web Archive paginator does not return page header")]
    #[allow(dead_code)]
    WebArchivePaginator,
    #[error("Attempt to scan while domain is None")]
    DomainMissing,
    #[error("Attempt to resolve bad host")]
    UnexpectedHost,
}
