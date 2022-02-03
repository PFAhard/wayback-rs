use std::collections::HashSet;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Otx {
    url_list: Vec<OtxMap>,
    has_next: bool,
    full_size: u64,
}

#[derive(Deserialize, Debug)]
struct OtxMap {
    url: String,
}

impl OtxMap {
    fn consume(self) -> String {
        self.url
    }
}

impl Otx {
    /// Get a reference to the otx's has next.
    pub fn has_next(&self) -> bool {
        self.has_next
    }

    pub fn size(&self) -> u64 {
        self.full_size
    }

    /// Get a reference to the otx's url list.
    pub fn url_list(self) -> HashSet<String> {
        self.url_list.into_iter().map(OtxMap::consume).collect()
    }
}
