use std::collections::HashSet;

use reqwest::blocking::Client;

use crate::{
    structs::{Expensive, IntoFlag, SubsFlag, WaybackRs, Verbose},
    utils::{error_unwrapper, timing_decorator},
};

pub type FlowMember = Box<dyn FnOnce() -> HashSet<String>>;

pub fn into_flow_member<F: 'static>(f: F) -> FlowMember
where
    F: FnOnce() -> HashSet<String>,
{
    Box::new(f)
}

pub type Flow = Vec<FlowMember>;

pub trait IntoFlow {
    fn into_flow(
        client: Client,
        domain: String,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<String>,
        batch: Vec<String>,
        verbose: Verbose
    ) -> Flow;
}

impl IntoFlow for Flow {
    fn into_flow(
        client: Client,
        domain: String,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<String>,
        batch: Vec<String>,
        verbose: Verbose,
    ) -> Flow {
        vec![
            {
                let domain = domain.clone();
                let client = client.clone();
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator("WA", || {
                            WaybackRs::get_wayback_url(&client, &domain, subs_flag)
                        }, verbose)
                    })
                })
            },
            {
                let domain = domain.clone();
                let client = client.clone();
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator("VT", || {
                            WaybackRs::get_virus_total_url(&client, &domain, vt_key.as_deref())
                        }, verbose)
                    })
                })
            },
            {
                let domain = domain.clone();
                let client = client.clone();
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator("OTX", || {
                            WaybackRs::get_otx_alienvault_url(&client, &domain)
                        }, verbose)
                    })
                })
            },
            expensive.select(
                {
                    let domain = domain.clone();
                    let client = client.clone();
                    into_flow_member(move || {
                        timing_decorator("CCbatch", || {
                            WaybackRs::get_batch_common_crawl(batch, &client, &domain, subs_flag)
                        }, verbose)
                    })
                },
                {
                    into_flow_member(move || {
                        error_unwrapper(|| {
                            timing_decorator("CC", || {
                                WaybackRs::get_common_crawl_url(&client, &domain, subs_flag, None)
                            }, verbose)
                        })
                    })
                },
            ),
        ]
    }
}
