use std::sync::Arc;

use reqwest::blocking::Client;

use crate::{
    structs::WaybackRs,
    utils::{error_unwrapper, timing_decorator}, blocking::structs::IntoFlag,
};

use super::{SubsFlag, Expensive, Verbose};

pub type FlowMember = Box<dyn FnOnce() -> Vec<String> + 'static + Sync + Send>;

pub fn into_flow_member<F>(f: F) -> FlowMember
where
    F: FnOnce() -> Vec<String> + 'static + Sync + Send,
{
    Box::new(f)
}

pub type Flow = Vec<FlowMember>;

#[allow(clippy::too_many_arguments)]
pub trait IntoFlow {
    fn into_flow(
        client: Arc<Client>,
        domain: Arc<String>,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<String>,
        batch: Vec<String>,
        expensive_threads: u8,
        verbose: Verbose,
    ) -> Flow;
}

impl IntoFlow for Flow {
    fn into_flow(
        client: Arc<Client>,
        domain: Arc<String>,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<String>,
        batch: Vec<String>,
        expensive_threads: u8,
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
                            WaybackRs::get_virus_total_url(
                                &client,
                                &domain,
                                vt_key.as_deref(),
                            )
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
                            WaybackRs::get_otx_alienvault_url(&client,&domain)
                        }, verbose)
                    })
                })
            },
            expensive.select({
                let domain = domain.clone();
                let client = client.clone();
                into_flow_member(move || {
                        timing_decorator("CCbatch", || {
                            WaybackRs::get_batch_common_crawl(
                                batch,
                                &client,
                                &domain,
                                expensive_threads,
                                subs_flag,
                            )
                    }, verbose)
                })
            }, {
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator("CC", || {
                            WaybackRs::get_common_crawl_url(
                                &client,
                                &domain,
                                subs_flag,
                                None,
                            )
                        }, verbose)
                    })
                })
            }),
        ]
    }
}
