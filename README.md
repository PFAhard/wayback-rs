# wayback-rs

[![crates.io](https://img.shields.io/crates/v/wayback)](https://crates.io/crates/wayback)
[![Documentation](https://docs.rs/wayback/badge.svg)](https://docs.rs/wayback)
![lang](https://img.shields.io/github/languages/top/PFAhard/wayback-rs)

Rust version of tomnomnom/waybackurls

Command line interface for fetching url from Wayback Machine, CommonCrawl, VirusTotal.

Update v1.0.0:
Increase productivity:

In results |orig | v0.3.1 | v1 | v1 async | v1 threads | v1 threads + async | expensive
---|---|---|---|---|---|---|---
6 urls |553.0ms±90.9ms|1.046s±0.146s|1.864s±0.270s|1.822s±0.347s|935.0ms±203.4ms| 1.582s±0.197s | 9,222(1.567)
~0,7kk urls | | | 52.402s±5.435s|||34.735s±8.625s
15 targets 90k |26.789s±3.368s||||6.747s±2.293s|5.726s±1.216s

Install:

1. git clone https://github.com/PFAhard/wayback-rs.git
2. cd wayback-rs
3. cargo build --release
4. cp target/release/wayback path_to_bin_path

```text
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
```
