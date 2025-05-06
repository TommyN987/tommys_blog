use std::sync::Arc;

use axum::{Router, extract::Request};
use tokio::{net::TcpListener, signal};
use tower_http::trace::TraceLayer;
use tracing::info_span;

use crate::{
    api::{health, post},
    domain::service::Service,
};

#[derive(Debug, Clone)]
pub struct AppState<S: Service> {
    pub service: Arc<S>,
}

impl<S: Service> AppState<S> {
    pub fn new(service: S) -> Self {
        Self {
            service: Arc::new(service),
        }
    }

    pub fn service(&self) -> &Arc<S> {
        &self.service
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

pub struct HttpServer {
    router: Router,
    listener: TcpListener,
}

impl HttpServer {
    pub async fn try_new<S: Service + 'static>(
        service: S,
        config: HttpServerConfig<'_>,
    ) -> Result<Self, anyhow::Error> {
        let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let uri = request.uri().to_string();
            info_span!("http_request", method = ?request.method(), uri)
        });

        let state = AppState::new(service);

        let router = Router::new()
            .merge(health::routes::<S>())
            .merge(post::routes::<S>())
            .layer(trace_layer)
            .with_state(state);

        let addr = format!("127.0.0.1:{}", config.port);

        let listener = TcpListener::bind(&addr).await?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_target(false)
            .with_writer(std::io::stdout)
            .init();

        tracing::info!("🔧 Tracing initialized");

        axum::serve(self.listener, self.router.into_make_service())
            .with_graceful_shutdown(Self::shutdown_signal())
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
}
