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
//! - HTTP 服务器（Axum + Tower 中间件）
//! - 数据库 ORM（SeaORM，支持 MySQL / PostgreSQL / SQLite）
//! - 结构化日志（Tracing + 请求追踪）
//! - 配置管理（YAML + 环境变量 + 命令行参数）
//! - 参数校验（Validator + axum-valid）
//! - 统一错误处理与 API 响应格式
//! - 分布式 ID 生成与密码安全
//!
//! ## 快速开始
//!
//! ```bash
//! cd example/web-starter
//! APP_NAME=example-web-starter cargo run
//! ```
//!
//! 后续模块将逐步从 `example/` 迁移到此根 crate。

/// 占位入口函数。
///
/// 当前仅输出 "Hello, world!"，实际服务启动请参考
/// `example/web-starter` 子 crate。
fn main() {
    println!("Hello, world!");
}
