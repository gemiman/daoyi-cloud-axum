//! JWT 认证模块。
//!
//! 使用 HMAC-SHA256 算法进行 Token 的编解码，实现无状态的用户认证。
//! 通过 [`JWT`] 结构体封装 `jsonwebtoken` crate，提供简单的 `encode`/`decode` API。
//!
//! ## 核心类型
//!
//! | 类型 | 说明 |
//! |------|------|
//! | [`Principal`] | 认证主体，包含用户 ID 和名称 |
//! | [`Claims`] | JWT 声明集（jti/sub/aud/iss/iat/exp） |
//! | [`JwtConfig`] | JWT 配置（密钥/过期时间/发行者/受众） |
//! | [`JWT`] | Token 编解码器 |
//!
//! ## 使用方式
//!
//! ```rust,ignore
//! use daoyi_axum_app::app::auth::jwt::default_jwt;
//!
//! // 编码：生成 Token
//! let token = default_jwt().encode(principal)?;
//!
//! // 解码：验证并提取 Principal
//! let principal = default_jwt().decode(&token)?;
//! ```

pub mod middleware;

use crate::app::auth::Principal;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::sync::LazyLock;
use std::time::Duration;

/// 默认密钥，用于开发环境。
/// 生产环境应通过 [`JwtConfig`] 配置更安全的密钥。
const DEFAULT_SECRET: &str = "12345678";

/// 全局默认 JWT 实例。
static DEFAULT_JWT_INSTANCE: LazyLock<JWT> = LazyLock::new(|| JWT::default());

/// JWT 声明集（Claims）。
///
/// 符合 JSON Web Token 标准声明的结构，包含以下标准字段：
///
/// | 字段 | 全称 | 说明 |
/// |------|------|------|
/// | `jti` | JWT ID | Token 唯一标识（使用 xid 生成） |
/// | `sub` | Subject | 认证主体（格式：`{id}:{name}`） |
/// | `aud` | Audience | Token 受众 |
/// | `iss` | Issuer | Token 发行者 |
/// | `iat` | Issued At | 签发时间（Unix 时间戳） |
/// | `exp` | Expiration | 过期时间（Unix 时间戳） |
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Token 唯一标识（xid）。
    jti: String,
    /// 认证主体，格式 `{id}:{name}`。
    sub: String,
    /// Token 受众。
    aud: String,
    /// Token 发行者。
    iss: String,
    /// 签发时间（Unix 时间戳）。
    iat: u64,
    /// 过期时间（Unix 时间戳）。
    exp: u64,
}

/// JWT 配置参数。
///
/// 通过实现 `Default` 提供开发环境的默认值，
/// 生产环境建议通过 YAML 配置或环境变量覆盖。
#[derive(Debug)]
pub struct JwtConfig {
    /// 签名密钥。
    pub secret: Cow<'static, str>,
    /// Token 有效期。
    pub expiration: Duration,
    /// Token 受众标识。
    pub audience: String,
    /// Token 发行者标识。
    pub issuer: String,
}

impl Default for JwtConfig {
    /// 返回开发环境的默认 JWT 配置。
    ///
    /// - 密钥：`"12345678"`
    /// - 过期时间：3600 秒（1 小时）
    fn default() -> Self {
        Self {
            secret: Cow::Borrowed(DEFAULT_SECRET),
            expiration: Duration::from_secs(3600),
            audience: "audience".to_string(),
            issuer: "issuer".to_string(),
        }
    }
}

/// JWT 编解码器。
///
/// 封装 `jsonwebtoken` crate 的编码/解码操作，使用 HMAC-SHA256 算法。
/// 内部维护 `EncodingKey`、`DecodingKey`、`Header`、`Validation` 等状态。
#[derive(Debug)]
pub struct JWT {
    /// HMAC-SHA256 编码密钥。
    encode_secret: EncodingKey,
    /// HMAC-SHA256 解码密钥。
    decode_secret: DecodingKey,
    /// JWT Header，固定为 HS256 算法。
    header: Header,
    /// Token 校验配置（验证签发者、受众、必需声明等）。
    validation: Validation,
    /// Token 有效期时长。
    expiration: Duration,
    /// Token 受众标识（用于校验）。
    audience: String,
    /// Token 发行者标识（用于校验）。
    issuer: String,
}

impl JWT {
    /// 根据配置创建新的 JWT 编解码器。
    ///
    /// 使用 HMAC-SHA256 算法，配置的密钥同时用于编码和解码。
    /// `Validation` 中设置了必须存在的声明字段：
    /// `jti`、`sub`、`aud`、`iss`、`iat`、`exp`。
    pub fn new(config: JwtConfig) -> Self {
        let secret = config.secret.as_bytes();
        let header = Header::new(Algorithm::HS256);
        let mut validation = Validation::new(Algorithm::HS256);
        validation.set_issuer(&[&config.issuer]);
        validation.set_audience(&[&config.audience]);
        validation.set_required_spec_claims(&["jti", "sub", "aud", "iss", "iat", "exp"]);
        Self {
            encode_secret: EncodingKey::from_secret(secret),
            decode_secret: DecodingKey::from_secret(secret),
            header,
            validation,
            expiration: config.expiration,
            audience: config.audience,
            issuer: config.issuer,
        }
    }

    /// 将 [`Principal`] 编码为 JWT Token 字符串。
    ///
    /// 自动填充 `jti`（xid）、`sub`（`{id}:{name}` 格式）以及时间戳。
    ///
    /// ## 错误
    ///
    /// 序列化失败时返回 `anyhow::Error`。
    pub fn encode(&self, principal: Principal) -> anyhow::Result<String> {
        let current_timestamp = get_current_timestamp();
        let claims = Claims {
            jti: xid::new().to_string(),
            sub: format!(
                "{}:{}:{}",
                principal.tenant_id, principal.id, principal.name
            ),
            aud: self.audience.clone(),
            iss: self.issuer.clone(),
            iat: current_timestamp,
            exp: current_timestamp.saturating_add(self.expiration.as_secs()),
        };
        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_secret,
        )?)
    }

    /// 解码并验证 JWT Token，返回认证主体。
    ///
    /// 验证包括：签名有效性、过期时间、签发者、受众和必需声明字段。
    ///
    /// ## 错误
    ///
    /// - Token 过期、签名无效等由 `jsonwebtoken` 返回的错误
    /// - `sub` 字段格式不为 `{id}:{name}` 时返回解析错误
    pub fn decode(&self, token: &str) -> anyhow::Result<Principal> {
        let claims: Claims =
            jsonwebtoken::decode(token, &self.decode_secret, &self.validation)?.claims;
        let mut parts = claims.sub.splitn(3, ':');
        let tenant_id = parts.next().unwrap().parse::<i64>()?;
        let id = parts.next().unwrap().parse::<i64>()?;
        let name = parts.next().unwrap().to_string();
        Ok(Principal {
            tenant_id,
            id,
            name,
        })
    }
}

impl Default for JWT {
    /// 使用 [`JwtConfig::default()`] 创建默认 JWT 编解码器。
    fn default() -> Self {
        Self::new(JwtConfig::default())
    }
}

/// 获取全局默认 JWT 实例的引用。
///
/// 使用 [`LazyLock`] 延迟初始化，线程安全。
/// 适用于所有处理器共享同一个 JWT 编解码器的场景。
///
/// ## 示例
///
/// ```rust,ignore
/// let token = default_jwt().encode(principal)?;
/// ```
pub fn default_jwt() -> &'static JWT {
    &DEFAULT_JWT_INSTANCE
}
