//! 密码哈希与验证模块。
//!
//! 基于 [`bcrypt`] 提供密码的哈希存储与验证功能。
//! 使用 bcrypt 默认的 cost 参数（`DEFAULT_COST`），在安全性和性能之间取得平衡。
//!
//! ## 安全性说明
//!
//! bcrypt 自带盐值（salt），每次哈希结果不同，无需额外管理盐值。

/// 对明文密码进行 bcrypt 哈希。
///
/// 使用 `bcrypt::DEFAULT_COST`（当前为 12），每次调用均会生成随机盐值，
/// 因此相同密码的两次哈希结果不同。
pub fn hash_passwd(passwd: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(passwd, bcrypt::DEFAULT_COST)?)
}

/// 验证明文密码是否与已哈希的密码匹配。
///
/// 内部从哈希字符串中提取盐值，无需额外参数。
///
/// ## 返回值
///
/// - `Ok(true)`：密码匹配
/// - `Ok(false)`：密码不匹配
/// - `Err(_)`：哈希字符串格式错误等异常情况
pub fn verify_passwd(passwd: &str, hashed_passwd: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(passwd, hashed_passwd)?)
}
