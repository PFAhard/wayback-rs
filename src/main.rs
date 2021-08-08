use wayback::{*};
use std::io::{Write,stdout};

fn main() {
    let (domains, subs_flag, api_key) = args::args_parser();
    let mut result: Vec<String> = vec![];

    let _ph = domains.iter().map(|domain| {
        if let Some(mut wb_result) = get_wayback_url(domain, subs_flag) {
            result.append(&mut wb_result);
        }
        if let Some(mut cc_result) = get_common_crawl_url(domain, subs_flag) {
            result.append(&mut cc_result);
        }
        if let Some(api_key) = &api_key {
            if let Some(mut vt_result) = get_virus_total_url(domain, api_key) {
                result.append(&mut vt_result);
            }
        }
        if let Some(mut otx_result) = get_otx_alienvault_url(domain) {
            result.append(&mut otx_result);
        }
    }).collect::<Vec<()>>();
    let _ph = result
        .into_iter()
        .map(|x| { match writeln!(&mut stdout(), "{}",x) {
            Ok(_ok) => {}
            Err(_err) => {/*
                TO-DO: Maybe use only with debug flag to minimize error-spam
                */}
        };})
        .collect::<Vec<()>>();
}