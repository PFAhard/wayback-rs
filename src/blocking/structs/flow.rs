use std::collections::HashSet;

use ureq::Agent;

use crate::{
    structs::{Expensive, SubsFlag, Verbose, WaybackRs},
    utils::{error_unwrapper, timing_decorator},
};

use super::IntoFlag;

pub(crate) struct Flow;

impl Flow {
    pub(crate) fn run(
        client: &Agent,
        domain: &str,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<&str>,
        batch: Vec<String>,
        verbose: Verbose,
    ) -> HashSet<String> {
        let mut urls = HashSet::new();
        urls.extend(error_unwrapper(|| {
            timing_decorator(
                "WA",
                || WaybackRs::get_wayback_url(client, domain, subs_flag),
                verbose,
            )
        }));
        urls.extend(error_unwrapper(|| {
            timing_decorator(
                "VT",
                || WaybackRs::get_virus_total_url(client, domain, vt_key),
                verbose,
            )
        }));
        urls.extend(error_unwrapper(|| {
            timing_decorator(
                "OTX",
                || WaybackRs::get_otx_alienvault_url(client, domain),
                verbose,
            )
        }));
        urls.extend(if expensive.into_flag() {
            timing_decorator(
                "CCbatch",
                || WaybackRs::get_batch_common_crawl(client, batch, domain, subs_flag),
                verbose,
            )
        } else {
            error_unwrapper(|| {
                timing_decorator(
                    "CC",
                    || WaybackRs::get_common_crawl_url(client, domain, subs_flag, None),
                    verbose,
                )
            })
        });
        urls
    }
}
