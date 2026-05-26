//! 认证与授权模块。
//!
//! 定义认证主体 [`Principal`] 并提供基于 JWT 的认证实现。
//!
//! ## 核心类型
//!
//! | 类型 | 所在模块 | 说明 |
//! |------|----------|------|
//! | [`Principal`] | `auth` | 认证主体（tenant_id/id/name） |
//! | [`JWT`] | `auth::jwt` | HMAC-SHA256 Token 编解码器 |
//! | [`JwtConfig`] | `auth::jwt` | JWT 配置参数（密钥/过期时间/签发者/受众） |
//! | [`Claims`] | `auth::jwt` | JWT 声明集（jti/sub/aud/iss/iat/exp） |
//! | [`JWTAuth`] | `auth::jwt::middleware` | Bearer Token 认证中间件 |
//!
//! ## 认证流程
//!
//! ```text
//! 客户端
//!   ├── POST /api/auth/login  → bcrypt 验证密码 → 签发 JWT Token
//!   └── 后续请求 + Authorization: Bearer <token>
//!                              → JWTAuth 中间件 → decode → Principal
//!                              → 注入 Extensions → Handler 可通过 Extension 获取
//! ```
//!
//! ## 使用示例
//!
//! ```rust,ignore
//! use daoyi_axum_app::app::auth::Principal;
//! use daoyi_axum_app::app::auth::jwt::{JWT, JwtConfig, default_jwt};
//!
//! // 签发 Token
//! let principal = Principal { tenant_id: 0, id: 1, name: "admin".into() };
//! let token = default_jwt().encode(principal)?;
//!
//! // 验证 Token（在 Handler 中通过 Extension 获取）
//! async fn handler(Extension(principal): Extension<Principal>) {
//!     println!("当前用户: {} (ID: {})", principal.name, principal.id);
//! }
//! ```

use serde::Serialize;

pub mod jwt;

/// 认证主体，解码 token 后得到的用户身份信息。
///
/// 由 JWT 中间件在解码成功后注入到请求的 [`Extensions`](axum::http::Extensions) 中，
/// Handler 可通过 [`Extension<Principal>`](axum::Extension) 提取器获取。
///
/// ## 字段
///
/// | 字段 | 类型 | 说明 |
/// |------|------|------|
/// | `tenant_id` | `i64` | 租户 ID（多租户隔离标识） |
/// | `id` | `i64` | 用户唯一 ID |
/// | `name` | `String` | 用户名称（便于日志输出和展示） |
///
/// ## 序列化
///
/// 序列化为 camelCase JSON 格式，可直接作为 API 响应返回。
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Principal {
    /// 租户 ID，用于多租户场景的数据隔离。默认 `0` 表示全局租户。
    pub tenant_id: i64,
    /// 用户唯一 ID（通常来自数据库主键或分布式 ID）。
    pub id: i64,
    /// 用户展示名称。
    pub name: String,
}
