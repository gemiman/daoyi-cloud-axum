//! HTTP 服务器模块。
//!
//! 构建并启动基于 Axum 的 HTTP 服务器，集成 TraceLayer、CORS、超时、
//! 请求体限制、路径规范化等中间件。通过 `into_make_service_with_connect_info`
//! 获取客户端真实 IP 地址。

use crate::app::AppState;
use crate::latency::LatencyOnResponse;
use axum::Router;
use axum::extract::{ConnectInfo, DefaultBodyLimit, Request};
use axum::http::StatusCode;
use bytesize::ByteSize;
use daoyi_axum_config::config::ServerConfig;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors;
use tower_http::cors::CorsLayer;
use tower_http::normalize_path::NormalizePathLayer;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

/// HTTP 服务器。
///
/// 封装了中间件组装、端口绑定与服务启动逻辑。
pub struct Server {
    /// 服务器配置的静态引用（全局配置单例）。
    config: &'static ServerConfig,
}

impl Server {
    /// 创建一个新的 `Server` 实例。
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }

    /// 启动 HTTP 服务器，绑定到 `0.0.0.0:{port}` 并开始监听。
    ///
    /// ## 流程
    ///
    /// 1. 调用 [`build_router`] 组装中间件
    /// 2. 绑定 TCP 端口
    /// 3. 进入服务循环，直到进程终止
    ///
    /// ## 注意事项
    ///
    /// 使用 [`into_make_service_with_connect_info::<SocketAddr>`] 以便
    /// 中间件可通过 [`ConnectInfo`] 提取客户端 IP。
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

    /// 组装中间件链。
    ///
    /// 中间件按从外到内的顺序应用（layer 顺序即为执行顺序）：
    ///
    /// | 顺序 | 中间件 | 说明 |
    /// |------|--------|------|
    /// | 1 | `TimeoutLayer` | 120s 超时保护 |
    /// | 2 | `DefaultBodyLimit` | 2 GiB 请求体上限 |
    /// | 3 | `TraceLayer` | 结构化日志（请求 ID、IP、耗时） |
    /// | 4 | `CorsLayer` | 允许所有来源的跨域请求 |
    /// | 5 | `NormalizePathLayer` | 去除 URL 尾部斜杠 |
    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        // 请求追踪层：为每个请求生成唯一 xid，记录 IP、方法、路径
        let tracing = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let id = xid::new();
                let ip = request
                    .extensions()
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|connect_info| connect_info.0.ip())
                    .unwrap_or_else(|| std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED));
                tracing::info_span!(
                    "HTTP Request",
                    id = %id,
                    ip = %ip,
                    method = %method,
                    path = %path
                )
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);

        // 超时保护：超过 120s 未响应则返回 408
        let timeout =
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(120));

        // 请求体大小限制：最大 2 GiB
        let body_limit = DefaultBodyLimit::max(ByteSize::gib(2).as_u64() as usize);

        // CORS 配置：开发/测试阶段允许所有来源
        let cors = CorsLayer::new()
            .allow_origin(cors::Any)
            .allow_methods(cors::Any)
            .allow_headers(cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));

        // 路径规范化：自动去除尾部斜杠（`/api/` → `/api`）
        let normalize_path = NormalizePathLayer::trim_trailing_slash();

        Router::new()
            .merge(router)
            .layer(timeout)
            .layer(body_limit)
            .layer(tracing)
            .layer(cors)
            .layer(normalize_path)
            .with_state(state)
    }
}
