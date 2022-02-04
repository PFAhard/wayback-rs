use std::{collections::HashSet, rc::Rc};

use crate::{
    structs::{Expensive, IntoFlag, SubsFlag, Verbose, WaybackRs},
    utils::{error_unwrapper, timing_decorator},
};

pub(crate) type FlowMember = Box<dyn FnOnce() -> HashSet<String>>;

pub(crate) fn into_flow_member<F: 'static>(f: F) -> FlowMember
where
    F: FnOnce() -> HashSet<String>,
{
    Box::new(f)
}

pub(crate) type Flow = Vec<FlowMember>;

pub(crate) trait IntoFlow {
    fn into_flow(
        domain: Rc<String>,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<String>,
        batch: Vec<String>,
        verbose: Verbose,
    ) -> Flow;
}

impl IntoFlow for Flow {
    #[inline]
    fn into_flow(
        domain: Rc<String>,
        subs_flag: SubsFlag,
        expensive: Expensive,
        vt_key: Option<String>,
        batch: Vec<String>,
        verbose: Verbose,
    ) -> Flow {
        vec![
            {
                let domain = domain.clone();
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator(
                            "WA",
                            || WaybackRs::get_wayback_url(&domain, subs_flag),
                            verbose,
                        )
                    })
                })
            },
            {
                let domain = domain.clone();
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator(
                            "VT",
                            || WaybackRs::get_virus_total_url(&domain, vt_key.as_deref()),
                            verbose,
                        )
                    })
                })
            },
            {
                let domain = domain.clone();
                into_flow_member(move || {
                    error_unwrapper(|| {
                        timing_decorator(
                            "OTX",
                            || WaybackRs::get_otx_alienvault_url(&domain),
                            verbose,
                        )
                    })
                })
            },
            expensive.select(
                {
                    let domain = domain.clone();
                    into_flow_member(move || {
                        timing_decorator(
                            "CCbatch",
                            || WaybackRs::get_batch_common_crawl(batch, &domain, subs_flag),
                            verbose,
                        )
                    })
                },
                {
                    into_flow_member(move || {
                        error_unwrapper(|| {
                            timing_decorator(
                                "CC",
                                || WaybackRs::get_common_crawl_url(&domain, subs_flag, None),
                                verbose,
                            )
                        })
                    })
                },
            ),
        ]
    }
}
