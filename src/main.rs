use std::net::Ipv4Addr;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let host: Host = args.host.into();

    println!("Host: {:?}, Timeout: {:?}s", host, args.timeout);
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(
        value_parser,
        help = "Host machine to ping. May be an IPv4 address or domain name."
    )]
    host: String,
    #[clap(short, long, default_value = "1", help = "Timeout interval (seconds)")]
    timeout: u32,
}

#[derive(Debug)]
enum Host {
    Ip(Ipv4Addr),
    Hostname(String),
}

impl From<String> for Host {
    fn from(host: String) -> Self {
        match host.parse::<Ipv4Addr>() {
            Ok(addr) => Host::Ip(addr),
            Err(_) => Host::Hostname(host),
        }
    }
}
