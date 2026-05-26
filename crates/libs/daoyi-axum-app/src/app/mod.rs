//! 应用启动与状态管理模块。
//!
//! 定义了全局共享的 [`AppState`] 以及服务启动入口 [`run`]。
//! `run` 按顺序完成：日志初始化 → ID 生成器初始化 → 数据库连接池初始化
//! → 服务器配置读取 → 中间件链组装 → HTTP 服务启动。
//!
//! ## 子模块
//!
//! | 模块 | 说明 |
//! |------|------|
//! | [`auth`] | JWT 认证（Principal / 编解码 / Bearer Token 中间件） |
//! | [`common`] | 通用数据结构（`PageParam`、`PageResult`） |
//! | [`database`] | 数据库连接池初始化（支持 MySQL/PostgreSQL/SQLite） |
//! | [`server`] | HTTP 服务器构建与启动（中间件链组装） |
//! | [`validation`] | 自定义参数校验函数（分页范围、手机号正则） |
//! | [`sea_orm_utils`] | SeaORM 扩展工具（预留扩展） |

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
/// 当前包含：
/// - **db** — SeaORM 数据库连接池（线程安全、可克隆）
///
/// ## 扩展方式
///
/// 后续可按需添加其他共享资源：
///
/// ```rust,ignore
/// pub struct AppState {
///     pub db: DatabaseConnection,
///     pub redis: redis::Client,          // 缓存
///     pub mq: MessageQueue,              // 消息队列
///     pub app_config: &'static AppConfig, // 应用配置（全局已有单例，通常无需冗余存储）
/// }
/// ```
///
/// ## 使用方式
///
/// ```rust,ignore
/// use axum::extract::State;
///
/// async fn handler(State(state): State<AppState>) {
///     let db = &state.db;
///     // 使用 db 进行数据库操作
/// }
/// ```
#[derive(Clone)]
pub struct AppState {
    /// SeaORM 数据库连接池。
    ///
    /// `DatabaseConnection` 内部使用 `Arc`，`Clone` 成本极低，
    /// 可在多线程间安全共享。
    pub db: DatabaseConnection,
}

impl AppState {
    /// 创建一个新的 `AppState` 实例。
    ///
    /// ## 参数
    ///
    /// - `db` — 已初始化的 SeaORM 数据库连接池
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

/// 应用启动入口。
///
/// 按以下顺序完成初始化后启动 HTTP 服务：
///
/// 1. **初始化 `tracing` 日志订阅器** — 基于 `tracing-subscriber`，
///    支持 `RUST_LOG` 环境变量控制日志级别
/// 2. **初始化分布式 ID 生成器** — 基于雪花算法，基准时间 2026-05-01
/// 3. **建立数据库连接池** — 通过 SeaORM 连接数据库，
///    连接池大小根据 CPU 核心数自适应
/// 4. **读取服务器配置** — 从全局配置单例中获取端口等参数
/// 5. **组装中间件链并启动 HTTP 服务** — 绑定 `0.0.0.0:{port}`，
///    进入事件循环直到进程终止
///
/// ## 参数
///
/// - `router` — 业务路由树，由调用方定义
///
/// ## 返回值
///
/// `anyhow::Result<()>` — 成功返回 `Ok(())`，
/// 失败可能在数据库连接、端口绑定等阶段发生
///
/// ## 示例
///
/// ```rust,ignore
/// use axum::{Router, routing::get};
/// use daoyi_axum_app::app;
///
/// let router = Router::new().route("/", get(|| async { "OK" }));
/// app::run(router).await?;
/// ```
pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    // 初始化结构化日志
    logger::init();
    tracing::info!("正在启动服务...");

    // 初始化雪花算法 ID 生成器
    id::init()?;

    // 建立数据库连接池并进行健康检查
    let db = database::init().await?;
    let state = AppState::new(db);

    // 从全局配置中获取服务器配置
    let server = server::Server::new(config::get().server());

    // 启动 HTTP 服务
    server.start(state, router).await?;

    Ok(())
}
