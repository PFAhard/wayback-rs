use std::collections::HashSet;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(crate) struct Otx {
    url_list: HashSet<OtxMap>,
    has_next: bool,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct OtxMap {
    url: String,
}

impl OtxMap {
    pub(crate) fn consume(self) -> String {
        self.url
    }
}

impl Otx {
    /// Get a reference to the otx's has next.
    pub(crate) fn has_next(&self) -> bool {
        self.has_next
    }

    /// Get a reference to the otx's url list.
    pub(crate) fn url_list(self) -> HashSet<String> {
        self.url_list.into_iter().map(OtxMap::consume).collect()
    }
}
