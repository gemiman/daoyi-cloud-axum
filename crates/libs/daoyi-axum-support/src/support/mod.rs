//! 支撑工具模块根节点。
//!
//! 组织所有基础设施子模块，对外提供统一导入路径。
//! 通过顶层 `support` 命名空间暴露所有工具类型与函数。

pub mod enumeration;
pub mod error;
pub mod id;
pub mod json;
pub mod latency;
pub mod logger;
pub mod passwd;
pub mod path;
pub mod query;
pub mod response;
pub mod serde;
pub mod valid;
