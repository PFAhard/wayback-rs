use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum WaybackError {
    #[error("VirusTotal key unprovided")]
    VirusTotalKey,
    #[error("VirusTotal return 204 No Content")]
    VirusTotalNoContent,
    #[error("Common Crawl: No Captures found for url")]
    CommonCrawlNoCaptures,
}
