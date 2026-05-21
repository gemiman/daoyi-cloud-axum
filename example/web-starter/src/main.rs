//! `web-starter` — daoyi-cloud-axum 的 Web 服务启动示例。
//!
//! 本 crate 演示如何使用 `axum` 搭建一个最小化的 HTTP 服务器，
//! 包括路由注册、配置加载、日志初始化等基础功能。

pub mod config;
pub mod logger;

use axum::{Router, debug_handler, routing};
use tokio::net::TcpListener;

/// 主入口函数。
///
/// 依次完成以下步骤：
/// 1. 初始化 `tracing` 日志订阅器
/// 2. 加载应用配置
/// 3. 注册路由
/// 4. 绑定端口并启动 HTTP 服务
#[tokio::main]
async fn main() {
    logger::init();
    let port = config::get().server.port();

    let router = Router::new().route("/", routing::get(index));

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    tracing::info!(
        "Listening on {}://{}",
        "http",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, router).await.unwrap();
}

/// 根路径 `/` 的 GET 处理器。
///
/// 返回一段静态欢迎文本。使用 `#[debug_handler]` 宏以便在
/// 编译期获得更好的类型错误提示。
#[debug_handler]
async fn index() -> &'static str {
    "Hello Daoyi Cloud Axum !"
}
