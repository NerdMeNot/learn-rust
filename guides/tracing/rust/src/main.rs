use tracing::{info, info_span, warn};

fn main() {
    // Print events to the terminal. without_time() keeps this page's output stable.
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .init();

    info!(port = 3000, "server starting");

    let span = info_span!("request", method = "GET", path = "/notes");
    let _guard = span.enter(); // everything until _guard is dropped is inside the span
    info!(count = 2, "listed notes");
    warn!(user = "ana", "slow response");
}
