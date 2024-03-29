use clap::Parser;
use color_eyre::eyre::Result;
use rping::RPing;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();
    let cancelled = Arc::new(AtomicBool::new(false));
    let c = cancelled.clone();
    ctrlc::set_handler(move || {
        c.store(true, Ordering::Relaxed);
        println!();
    })?;

    let args = Args::parse();
    let mut rping = RPing::new((args.host.clone(), 0u16), args.timeout, cancelled)?;
    rping.start(args.count)?;
    rping.dump_stats();

    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false).compact();
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Host machine to ping. May be an IPv4 address or domain name.
    host: String,

    #[arg(short = 'W', long, default_value = "1")]
    /// Timeout interval (seconds)
    timeout: u64,

    #[arg(short, long, default_value_t = u16::MAX)]
    /// Number of ping requests to send
    count: u16,
}
