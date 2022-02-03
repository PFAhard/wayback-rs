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
6 urls | 0,594(0.453) | 1,1(0.909) | 1,7878(1.53) | 1,674(1.392) | 0,936(0.823) | 2,356(1.422) | 9,222(1.567)
~0,7kk urls |
15 targets 90k |

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
