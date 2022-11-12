use clap::Parser;
use color_eyre::eyre::Result;
use rping::RPing;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing::instrument;

#[instrument]
fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();

    let args = Args::parse();
    let cancelled = Arc::new(AtomicBool::new(false));
    let mut rping = RPing::new((args.host.clone(), 0u16), args.timeout, cancelled.clone())?;
    ctrlc::set_handler(move || {
        cancelled.store(true, Ordering::Relaxed);
    })?;
    rping.start(args.count)?;
    rping.dump_stats();

    Ok(())
}

fn install_tracing() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
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
    timeout: i64,

    #[arg(short, long, default_value_t = u16::MAX)]
    /// Number of ping requests to send
    count: u16,
}
