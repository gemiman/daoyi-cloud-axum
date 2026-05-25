//! 应用启动与状态管理模块。
//!
//! 定义了全局共享的 [`AppState`] 以及服务启动入口 [`run`]。
//! `run` 按顺序完成日志初始化、数据库连接、配置加载、HTTP 服务启动等步骤。

pub mod auth;
pub mod common;
pub mod database;
pub mod sea_orm_utils;
pub mod server;
pub mod validation;

use axum::Router;
use daoyi_axum_config::config;
use daoyi_axum_support::support::{id, logger};
use sea_orm::DatabaseConnection;

/// 全局应用状态，通过 Axum 的 [`State`](axum::extract::State) 提取器在
/// 各处理器中共享。
///
/// 当前仅包含数据库连接池，后续可扩展缓存、消息队列等共享资源。
#[derive(Clone)]
pub struct AppState {
    /// SeaORM 数据库连接池。
    pub db: DatabaseConnection,
}

impl AppState {
    /// 创建一个新的 `AppState` 实例。
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

/// 应用启动入口。
///
/// 按以下顺序完成初始化：
/// 1. 初始化 `tracing` 日志订阅器
/// 2. 建立数据库连接并获取版本信息
/// 3. 读取服务器配置
/// 4. 绑定端口并启动 HTTP 服务
pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    logger::init();
    tracing::info!("正在启动服务...");

    id::init()?;

    // 初始化数据库连接池
    let db = database::init().await?;
    let state = AppState::new(db);
    // 从全局配置中获取服务器配置
    let server = server::Server::new(config::get().server());

    server.start(state, router).await?;

    Ok(())
}
