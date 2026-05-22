//! `web-starter` — daoyi-cloud-axum 的 Web 服务启动示例。
//!
//! 本 crate 演示如何使用 `axum` 搭建一个最小化的 HTTP 服务器，
//! 包括路由注册、配置加载、日志初始化等基础功能。

pub mod api;
pub mod app;
pub mod common;
pub mod config;
pub mod database;
pub mod demo;
pub mod error;
pub mod id;
pub mod json;
pub mod latency;
pub mod logger;
pub mod path;
pub mod query;
pub mod response;
pub mod sea_orm_utils;
pub mod serde;
pub mod server;
pub mod valid;
pub mod validation;

/// 主入口函数。
///
/// 依次完成以下步骤：
/// 1. 初始化 `tracing` 日志订阅器
/// 2. 加载应用配置
/// 3. 注册路由
/// 4. 绑定端口并启动 HTTP 服务
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run(api::create_router()).await
}
