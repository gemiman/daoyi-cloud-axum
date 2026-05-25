//! 认证与授权模块。
//!
//! 当前仅包含基于 JWT 的认证实现，包括：
//! - [`jwt::JWT`] — HMAC-SHA256 Token 编解码器
//! - [`jwt::JWTAuth`] — 基于 Authorization Header 的 Bearer Token 认证中间件

use serde::Serialize;

pub mod jwt;

/// 认证主体，解码 token 后得到的用户信息。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Principal {
    /// 租户 ID。
    pub tenant_id: i64,
    /// 用户 ID。
    pub id: i64,
    /// 用户名称。
    pub name: String,
}
