pub mod args;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::blocking::{Client};

//Limit for https://otx.alienvault.com/
//I can’t set a limit higher than 50. otx still gives out a limit of 50.
const LIMIT: u8 = 50;
fn client() -> Client {
    match Client::builder().timeout(None).build() {
        Ok(client) => client,
        Err(err) => {
            eprintln!("{:?}",err);
            Client::new()
        }
    }
}

pub fn get_wayback_url(domain: &String, subs_flag: bool) -> Option<Vec<String>> {
    let sub_wild_card = match subs_flag {
        true => "*.",
        false => "",
    };
    let mut urls = None;

    match client().get(format!("http://web.archive.org/cdx/search/cdx?url={}{}/*&output=json&collapse=urlkey", sub_wild_card, domain)).send() {
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

pub fn get_common_crawl_url(domain: &String, subs_flag: bool) -> Option<Vec<String>> {
    let sub_wild_card = match subs_flag {
        true => "*.",
        false => "",
    };
    let mut urls = None;

    match client().get(format!("http://index.commoncrawl.org/CC-MAIN-2018-22-index?url={}{}/*&output=json", sub_wild_card, domain)).send() {
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
                                                None => {/*
                                                    TO-DO: Maybe use only with debug flag to minimize error-spam
                                                    */}
                                            }
                                        }
                                        None => {/*
                                            TO-DO: Maybe use only with debug flag to minimize error-spam
                                            */}
                                    }
                                }
                                Err(err) => { eprintln!("{:?}",err); }
                            }
                        }).collect::<()>();
                    if !inner_urls.is_empty() {urls = Some(inner_urls)}
                },
                Err(err) => { eprintln!("{:?}",err); }
            }
        }
        Err(err) => { eprintln!("{:?}",err); }
    }

    urls
}

pub fn get_virus_total_url(domain: &String, api_key: &String) -> Option<Vec<String>> {
    let mut urls = None;

    match client().get(format!("https://www.virustotal.com/vtapi/v2/domain/report?apikey={}&domain={}", api_key, domain)).send() {
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
                                None => {/*
                                    TO-DO: Maybe use only with debug flag to minimize error-spam
                                    */}
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


//Is it possible to implement subs_flag on alientvault api? 
pub fn get_otx_alienvault_url(domain: &String) -> Option<Vec<String>> {
    let mut urls = Vec::<String>::new();
    let mut page: u16 = 1;
    let mut has_next = true;

    while has_next {
        match client().get(format!("https://otx.alienvault.com/api/v1/indicators/domain/{}/url_list?limit={}&page={}", domain, LIMIT, page)).send() {
            Ok(response) => {
                match response.json::<OTX>() {
                    Ok(otx) => {
                        has_next = otx.has_next;
                        page += 1;
                        let _ph = otx.url_list.into_iter().map(|instance| {
                            match instance.get("url") {
                                Some(url) => {
                                    match url.as_str() {
                                        Some(url_str) => {
                                            let url_string = url_str.to_string();
                                            urls.push(url_string)
                                        }
                                        None => {/*
                                            TO-DO: Maybe use only with debug flag to minimize error-spam
                                            */}
                                    }
                                }
                                None => {/*
                                    TO-DO: Maybe use only with debug flag to minimize error-spam
                                    */}
                            };
                        }).collect::<()>();
                    }
                    Err(err) => { eprintln!("{:?}",err);}
                }
            }
            Err(err) => { eprintln!("{:?}", err);}
        }
    }

    if !urls.is_empty() {Some(urls)} else {None}
}

//Struct for extracting otx.alienvault result
#[derive(Serialize, Deserialize)]
struct OTX {
    url_list: Vec<Value>,
    page_num: u16,
    limit: u8,
    paged: bool,
    has_next: bool,
    full_size: usize,
    actual_size: usize,
}

//Struct for extracting VirusTotal result
#[derive(Serialize, Deserialize)]
struct VT {
    url: String,
    positives: u32,
    total: u32,
    scan_date: String,
}