use notes_api::{config::Config, http, storage::SqliteStore};
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .without_time() // keeps the output on the page stable; remove it for real use
        .init();

    let config = Config::from_env()?;
    let store = SqliteStore::connect(&config.database_url).await?;
    let listener = TcpListener::bind(config.bind).await?;
    tracing::info!(addr = %listener.local_addr()?, "listening on");

    axum::serve(listener, http::router(store.clone()))
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    store.close().await;
    tracing::info!("stopped");
    Ok(())
}

/// Finishes when the process is asked to stop: Ctrl-C, or SIGTERM on Unix
/// (what `docker stop` and most process managers send).
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("could not install the Ctrl-C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("could not install the SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
    tracing::info!("shutdown signal received");
}
