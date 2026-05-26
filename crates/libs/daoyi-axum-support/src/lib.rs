//! `daoyi-axum-support` — 基础设施支撑 crate。
//!
//! 提供微服务开发中常用的工具模块，是 `daoyi-axum-app` 和
//! `daoyi-sea-orm-entity-demo` 的公共依赖。
//!
//! ## 模块总览
//!
//! | 模块 | 功能 | 核心能力 |
//! |------|------|----------|
//! | [`error`] | 统一错误处理 | `ApiError` 枚举 + `ApiResult<T>` 别名，自动映射 HTTP 状态码 |
//! | [`response`] | 统一响应格式 | `ApiResponse<T>` (code/msg/data) + `success()` / `fail()` 快捷构造 |
//! | [`enumeration`] | 通用枚举 | `Gender` 等，实现 SeaORM `DeriveActiveEnum` |
//! | [`id`] | 分布式 ID | 雪花算法，`init()` / `next_id()` / `next_id_str()` |
//! | [`passwd`] | 密码安全 | bcrypt 哈希/验证，同步/异步双版本 |
//! | [`valid`] | 参数校验 | `ValidQuery<T>` / `ValidPath<T>` / `ValidJson<T>` 提取器 |
//! | [`json`] / [`query`] / [`path`] | 自定义提取器 | 自动错误转换，集成 `HasValidate` |
//! | [`serde`] | 反序列化工具 | `deserialize_number` — 字符串/数字兼容 |
//! | [`logger`] | 结构化日志 | `init()` — tracing-subscriber，本地时间 + ISO 8601 时区 |
//! | [`latency`] | 请求耗时 | `LatencyOnResponse` — TraceLayer 响应回调 |
//!
//! ## 架构原则
//!
//! - **无状态** — 本 crate 不维护任何全局状态（除了日志订阅器）
//! - **无业务耦合** — 所有模块均为通用工具，不包含具体业务逻辑
//! - **错误友好** — 统一 `ApiError` 枚举覆盖所有层级，自动映射 HTTP 状态码
//! - **异步友好** — CPU 密集型操作（如 bcrypt）提供异步版本，通过 `spawn_blocking` 卸载
//!
//! ## 依赖关系
//!
//! ```text
//! daoyi-axum-app ──── daoyi-axum-support
//! daoyi-sea-orm-entity-demo ──── daoyi-axum-support
//! ```

pub mod support;
