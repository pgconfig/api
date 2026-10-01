//! `pgconfig-server`: serves the pgconfig API.

use std::net::{Ipv4Addr, SocketAddr};

use clap::Parser;
use tokio::net::TcpListener;

#[derive(Parser)]
#[command(name = "pgconfig-server", about = "Serves the pgconfig API")]
struct Args {
    /// Listen port
    #[arg(long, env = "PORT", default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, args.port));
    let listener = TcpListener::bind(address).await?;
    tracing::info!(version = %pgconfig::build::pretty(), %address, "PGConfig API");

    axum::serve(listener, pgconfig_server::app())
        .with_graceful_shutdown(shutdown())
        .await
}

/// Resolves on Ctrl-C, and on SIGTERM where the platform has it, so a
/// container stop finishes the requests in flight.
async fn shutdown() {
    let interrupt = async {
        let _ = tokio::signal::ctrl_c().await;
    };
    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut stream) => {
                stream.recv().await;
            }
            Err(_) => std::future::pending::<()>().await,
        }
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = interrupt => {}
        () = terminate => {}
    }
}
