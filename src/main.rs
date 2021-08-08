use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::blocking;
use clap::{Arg, App};

fn main() {
    let (domains, subs_flag, api_key) = args_parser();
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
    }).collect::<Vec<()>>();
    let _ph = result.into_iter().map(|x| {println!("{}",x)}).collect::<Vec<()>>();
}

fn get_wayback_url(domain: &String, subs_flag: bool) -> Option<Vec<String>> {
    let sub_wild_card = match subs_flag {
        true => "*.",
        false => "",
    };
    let mut urls = None;

    match blocking::get(format!("http://web.archive.org/cdx/search/cdx?url={}{}/*&output=json&collapse=urlkey", sub_wild_card, domain)) {
        Ok(response) => {
            match response.json::<Vec<Vec<String>>>() {
                Ok(response) => {
                    let wayback_urls: Vec<String> = response.into_iter().filter(|line|{
                            line != &vec!["urlkey","timestamp","original","mimetype","statuscode","digest","length"]
                        }).map(|line| {
                            line[2].clone()
                        }).collect();
                    urls = Some(wayback_urls);
                }
                Err(err) => { eprintln!("{:?}", err); }
            }
        }
        Err(err) => { eprintln!("{:?}", err); }
    }

    urls
}

fn get_common_crawl_url(domain: &String, subs_flag: bool) -> Option<Vec<String>> {
    let sub_wild_card = match subs_flag {
        true => "*.",
        false => "",
    };
    let mut urls = None;

    match blocking::get(format!("http://index.commoncrawl.org/CC-MAIN-2018-22-index?url={}{}/*&output=json", sub_wild_card, domain)) {
        Ok(response) => {
            match response.text() {
                Ok(response) => {
                    let mut inner_urls: Vec<String> = vec![];
                    let _ph = response
                        .split("\n")
                        .filter(|x| x != &"")
                        .map(|x| {
                            match serde_json::from_str::<Value>(x) {
                                Ok(instance) => {
                                    match instance.get("url") {
                                        Some(url) => {
                                            match url.as_str() {
                                                Some(url_str) => {
                                                    let url_string = url_str.to_string();
                                                    inner_urls.push(url_string)
                                                }
                                                None => { eprintln!("No one url_str in commoncrawl"); }
                                            }
                                        }
                                        None => { eprintln!("No one url in commoncrawl"); }
                                    }
                                }
                                Err(err) => { eprintln!("{:?}",err); }
                            }
                    });
                    if !inner_urls.is_empty() {urls = Some(inner_urls)}
                },
                Err(err) => { eprintln!("{:?}",err); }
            }
        }
        Err(err) => { eprintln!("{:?}",err); }
    }

    urls
}

fn get_virus_total_url(domain: &String, api_key: &String) -> Option<Vec<String>> {
    let mut urls = None;

    match blocking::get(format!("https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={}", api_key, domain)) {
        Ok(response) => {
            match response.text() {
                Ok(response) => {
                    match serde_json::from_str::<Value>(&response) {
                        Ok(res_val) => {
                            match res_val.get("detected_urls") {
                                Some(detected_urls) => {
                                    match serde_json::from_value::<Vec<VT>>(detected_urls.clone()) {
                                        Ok(vts) => {
                                            urls = Some(vts
                                                .into_iter()
                                                .map(|x| x.url)
                                                .collect());
                                        }
                                        Err(err) => {eprintln!("{:?}", err); }
                                    }
                                }
                                None => { eprintln!("No one url in VirusTotal or Api quote"); }
                            } 
                        }
                        Err(err) => {eprintln!("{:?}",err); }
                    }
                }
                Err(err) => { eprintln!("{:?}", err); }
            }
        }
        Err(err) => { eprintln!("{:?}", err); }
    }

    urls
}

#[derive(Serialize, Deserialize)]
struct VT {
    url: String,
    positives: u32,
    total: u32,
    scan_date: String,
}

fn args_parser() -> (Vec<String>, bool, Option<String>){
    let mut domains: Vec<String> = vec![];
    let mut subs_flag = false;
    let mut api_key = None;
    let matches = App::new("wayback-rs")
        .version("0.1.0")
        .author("pfapostol")
        .arg(Arg::with_name("domain")
            .short("d")
            .long("domain")
            .help("use for scan one domain")
            .takes_value(true)
        ).arg(Arg::with_name("subs_flag")
            .help("scan for subdomain also")
            .short("s")
        ).arg(Arg::with_name("list")
            .long("list")
            .short("l")
            .help("file with domains for scan")
            .takes_value(true)
        ).arg(Arg::with_name("vt_key")
            .long("vt-api-key")
            .short("vt")
            .help("virus total api key")
            .takes_value(true)
        ).get_matches();   
    if let Some(domain) = matches.value_of("domain") {
        domains.push(domain.to_string());
    }
    if matches.is_present("subs_flag") {
        subs_flag = true;
    }
    if let Some(list) = matches.value_of("list") {
        let data = std::fs::read_to_string(list).expect("Unable to read file");
        let _ph = data
            .split("\n")
            .map(|x| {
                domains.push(x.trim().to_string());
            })
            .collect::<Vec<()>>();
    }
    if let Some(vt_key) = matches.value_of("vt_key") {
        api_key = Some(vt_key.to_string())
    }
    (domains, subs_flag, api_key)
}