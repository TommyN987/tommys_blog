use axum::{Extension, Router};
use backend::{api, config::Config, db::postgres::Postgres};
use tokio::{self, net::TcpListener, signal};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .with_target(false)
        .with_writer(std::io::stdout)
        .init();

    tracing::info!("🔧 Tracing initialized");

    let config = Config::from_env();
    let postgres = Postgres::try_new(&config.database_url).await?;
    let app = Router::new()
        .merge(api::health::routes())
        .merge(api::post::routes())
        .layer(Extension(postgres.clone()));

    let addr = format!("127.0.0.1:{}", config.port);

    let listener = TcpListener::bind(&addr).await?;
    println!("🚀 Server running at http://{}", addr);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("🔌 Shutdown signal received.");
}
