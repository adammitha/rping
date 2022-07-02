use std::net::Ipv4Addr;

use clap::Parser;

fn main() {
    let args = Args::parse();

    let host: Host = args.host.into();

    println!("Host: {:?}", host);
}

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(value_parser)]
    host: String,
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

