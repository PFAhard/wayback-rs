use wayback::{*};
use std::io::{Write,stdout};

fn main() {
    let (domains, subs_flag, api_key) = args::args_parser();
    
    let result = scan_domains(domains, subs_flag, api_key);

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