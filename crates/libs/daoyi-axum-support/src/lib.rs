//! `daoyi-axum-support` — 基础设施支撑 crate。
//!
//! 提供微服务开发中常用的工具模块，包括：
//!
//! | 模块 | 功能 |
//! |------|------|
//! | [`error`] | 统一错误枚举与自动 HTTP 状态码映射 |
//! | [`response`] | 标准化 API 响应格式（code/msg/data） |
//! | [`enumeration`] | 通用枚举类型（如性别） |
//! | [`id`] | 雪花算法分布式 ID 生成 |
//! | [`valid`] | 校验型请求参数提取器 |
//! | [`json`] / [`query`] / [`path`] | 自定义提取器（自动错误转换） |
//! | [`passwd`] | bcrypt 密码哈希与验证 |
//! | [`serde`] | 通用反序列化工具 |
//! | [`logger`] | 结构化日志初始化 |
//! | [`latency`] | 请求耗时记录回调 |

pub mod support;
