use std::io;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init_logging(verbose: bool) {
    let env_filter = if let Ok(rust_log) = std::env::var("RUST_LOG") {
        EnvFilter::new(rust_log)
    } else if verbose {
        EnvFilter::new("jari=debug")
    } else {
        EnvFilter::new("jari=warn")
    };

    let log_format = std::env::var("JARI_LOG_FORMAT").unwrap_or_default();

    if log_format == "json" {
        let layer = fmt::layer()
            .json()
            .with_writer(io::stderr)
            .with_target(false);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(layer)
            .init();
    } else {
        let layer = fmt::layer().with_writer(io::stderr).with_target(false);

        tracing_subscriber::registry()
            .with(env_filter)
            .with(layer)
            .init();
    }
}
