use rping::RPing;
use clap::Parser;

fn main() {
    let args = Args::parse();
    println!("Resolving host {}", args.host);
    let rping = RPing::new(args.timeout, (args.host.clone(), 0u16)).unwrap();
    println!("Pinging host: {}", rping.host.ip());
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Host machine to ping. May be an IPv4 address or domain name.
    host: String,
    #[arg(short, long, default_value = "1")]
    /// Timeout interval (seconds)
    timeout: i64,
}
