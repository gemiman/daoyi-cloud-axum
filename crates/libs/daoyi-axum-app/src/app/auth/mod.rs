//! 认证与授权模块。
//!
//! 当前仅包含基于 JWT 的认证实现，包括：
//! - [`jwt::JWT`] — HMAC-SHA256 Token 编解码器
//! - [`jwt::JWTAuth`] — 基于 Authorization Header 的 Bearer Token 认证中间件

pub mod jwt;
