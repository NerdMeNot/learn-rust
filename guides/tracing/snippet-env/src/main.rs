use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

fn main() {
    // RUST_LOG picks the levels. With it unset (or invalid), show info and above.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(filter)
        .init();

    debug!(key = "notes", "cache miss");
    info!(port = 3000, "server starting");
    warn!(user = "ana", "slow response");
}
