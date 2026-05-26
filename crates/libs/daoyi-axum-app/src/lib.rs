//! `daoyi-axum-app` — 应用启动与核心功能 crate。
//!
//! 负责组装中间件链、数据库连接、JWT 认证等核心功能，
//! 并通过 [`app::run`] 提供一键启动 HTTP 服务的入口函数。
//!
//! ## 核心模块
//!
//! | 模块 | 功能 |
//! |------|------|
//! | [`app:: mod `] | 应用启动入口（`AppState` + `run()`） |
//! | [`app::server`] | HTTP 服务器构建与启动（中间件链组装） |
//! | [`app::database`] | 数据库连接池初始化（自适应 CPU 核心数） |
//! | [`app::common`] | 通用数据结构（`PageParam`、`PageResult`） |
//! | [`app::auth`] | JWT 认证（`Principal` + 编解码 + Bearer Token 中间件） |
//! | [`app::validation`] | 自定义参数校验函数（分页范围、手机号正则） |
//! | [`app::sea_orm_utils`] | SeaORM 扩展工具（扩展预留） |
//!
//! ## 启动流程
//!
//! ```text
//! app::run(router)
//!   ├── logger::init()           — 初始化结构化日志（Tracing）
//!   ├── id::init()               — 初始化雪花算法 ID 生成器
//!   ├── database::init()         — 建立 SeaORM 数据库连接池
//!   ├── config::get().server()   — 读取全局服务器配置
//!   └── server::start()          — 组装中间件链，启动 HTTP 服务
//!         ├── TimeoutLayer (120s)
//!         ├── DefaultBodyLimit (2 GiB)
//!         ├── TraceLayer (xid/IP/userId/耗时)
//!         ├── CorsLayer (跨域)
//!         ├── NormalizePathLayer (路径规范化)
//!         └── Router + AppState → Handler
//! ```
//!
//! ## 依赖关系
//!
//! ```text
//! daoyi-axum-app
//! ├── daoyi-axum-support  — 基础设施（错误处理、ID 生成、密码、日志）
//! └── daoyi-axum-config   — 配置管理（YAML + 环境变量）
//! ```

pub mod app;
