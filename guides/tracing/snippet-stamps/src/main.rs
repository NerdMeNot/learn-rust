use tracing::info;

fn main() {
    // No without_time(): every line starts with the time it was logged.
    tracing_subscriber::fmt().with_target(false).init();

    info!(port = 3000, "server starting");
}
