use crate::app::AppState;
use crate::config::ServerConfig;
use axum::Router;
use axum::extract::Request;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.port())).await?;
        let addr = listener.local_addr()?;

        tracing::info!("监听地址 {}://{}", "http", addr);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                tracing::info_span!("HTTP Request", id = %id, method = %method, path = %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(DefaultOnResponse::new().level(tracing::Level::INFO));
        Router::new().merge(router).layer(tracing).with_state(state)
    }
}
