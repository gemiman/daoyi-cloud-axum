//! `daoyi-cloud-axum` 根 crate。
//!
//! 本 crate 是 Cargo Workspace 的根 crate，当前作为占位入口，暂无功能实现。
//! 实际应用代码位于 `example/` 目录下的子 crate 中。
//!
//! ## 项目定位
//!
//! 本项目目标是提供一个基于 **Axum + SeaORM + MySQL** 的
//! Rust 云原生微服务脚手架，集成以下核心能力：
//!
//! - HTTP 服务器（Axum + Tower 中间件：超时、限流、CORS、日志追踪）
//! - 数据库 ORM（SeaORM，支持 MySQL / PostgreSQL / SQLite，连接池自适应 CPU 核心数）
//! - 结构化日志（Tracing + 请求追踪 ID + 本地时间 + 时区偏移）
//! - 配置管理（YAML 配置文件 + 环境变量覆盖 + 命令行参数指定路径）
//! - 参数校验（Validator + axum-valid：Query / Path / JSON Body）
//! - 统一错误处理（自动映射 HTTP 状态码，JSON 格式错误信息）
//! - 统一 API 响应（code / msg / data 三字段标准化输出）
//! - 分布式 ID 生成（雪花算法，基准时间 2026-05-01）
//! - 密码安全（bcrypt 哈希 / 验证，支持同步和异步两种模式）
//! - JWT 无状态认证（HMAC-SHA256 编解码 + Bearer Token 中间件）
//! - 分页支持（内置通用分页参数与分页响应，含参数校验）
//! - 静态资源嵌入（rust-embed，支持 SPA 部署模式）
//!
//! ## 项目结构
//!
//! ```text
//! crates/
//! ├── libs/
//! │   ├── daoyi-axum-config/    — 配置管理（YAML + 环境变量 + CLI）
//! │   ├── daoyi-axum-support/   — 基础设施（错误/响应/ID/密码/日志/校验）
//! │   └── daoyi-axum-app/       — 应用核心（服务器/数据库/JWT/分页/校验）
//! └── sea-orm-entities/
//!     └── daoyi-sea-orm-entity-demo/ — 数据库 Entity 模型
//!
//! example/
//! └── web-starter/              — 完整 Web 服务示例
//! ```
//!
//! ## 快速开始
//!
//! ```bash
//! cd example/web-starter
//! APP_NAME=example-web-starter cargo run
//! ```
//!
//! 详细文档请参阅项目根目录的 [README](../../README.md)。

/// 占位入口函数。
///
/// 当前仅输出 "Hello, world!"，实际服务启动请参考
/// `example/web-starter` 子 crate。
///
/// 完整的 Web 服务启动示例参见
/// [`web_starter::api::create_router`](../example/web-starter/src/main.rs)
/// 中的 `app::run()` 调用方式。
fn main() {
    println!("Hello, world!");
}
