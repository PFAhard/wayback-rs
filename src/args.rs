use clap::{Arg, App};

pub(crate) fn args_parser() -> (Vec<String>, bool, Option<String>){
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
        if domain.is_empty() {panic!("domain is empty!")};
        domains.push(domain.to_string());
    }
    if matches.is_present("subs_flag") {
        subs_flag = true;
    }
    if let Some(list) = matches.value_of("list") {
        let data = std::fs::read_to_string(list).expect("Unable to read file");
        let _ph = data
            .split("\n")
            .filter(|x| !x.is_empty())
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