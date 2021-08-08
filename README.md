# wayback-rs
Rust version of tomnomnom/waybackurls

Command line interface for fetching url from Wayback Machine, CommonCrawl, VirusTotal.

Install:
1. git clone https://github.com/PFAhard/wayback-rs.git
2. cd wayback-rs
3. cargo build --release
4. cp target/release/wayback path_to_bin_path


```
USAGE:
    wayback [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -s               scan for subdomain also
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>        use for scan one domain
    -l, --list <list>            file with domains for scan
    -v, --vt-api-key <vt_key>    virus total api key