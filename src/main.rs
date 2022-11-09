use rping::RPing;
use clap::Parser;

fn main() {
    let args = Args::parse();
    let rping = RPing::new((args.host.clone(), 0u16), args.timeout).unwrap();
    rping.start(args.count);
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Host machine to ping. May be an IPv4 address or domain name.
    host: String,

    #[arg(short, long, default_value = "1")]
    /// Timeout interval (seconds)
    timeout: i64,

    #[arg(short = 'n', long)]
    /// Number of ping requests to send
    count: Option<u64>,
}
