//! 密码哈希与验证模块。
//!
//! 基于 [`bcrypt`] 提供密码的哈希存储与验证功能。
//! 使用 bcrypt 默认的 cost 参数（`DEFAULT_COST` = 12，即 2^12 ≈ 4096 次迭代），
//! 在安全性和性能之间取得平衡。
//!
//! ## 安全性说明
//!
//! bcrypt 自带盐值（salt），每次哈希结果不同，无需额外管理盐值。
//! cost 越高越安全，但耗时也越长。当前默认值 12 在绝大多数
//! 场景下可同时满足安全性与用户体验要求。
//!
//! ## 异步支持
//!
//! bcrypt 是 CPU 密集型操作（单次约 250-350ms），直接调用会阻塞
//! tokio 工作线程。本模块提供 [`hash_passwd_async`] 和
//! [`verify_passwd_async`] 两个异步封装，内部通过
//! [`tokio::task::spawn_blocking`] 将计算卸载到专用阻塞线程池，
//! 确保异步运行时不被阻塞。
//!
//! **推荐在 axum handler 等异步上下文中优先使用 `_async` 版本。**

/// 对明文密码进行 bcrypt 哈希（同步版本）。
///
/// 使用 `bcrypt::DEFAULT_COST`（当前为 12），每次调用均会生成随机盐值，
/// 因此相同密码的两次哈希结果不同。
///
/// > **注意**：此函数是 CPU 密集型操作，在异步上下文中会阻塞 tokio
/// > 工作线程。如需在 axum handler 中调用，请改用 [`hash_passwd_async`]。
pub fn hash_passwd(passwd: &str) -> anyhow::Result<String> {
    Ok(bcrypt::hash(passwd, bcrypt::DEFAULT_COST)?)
}

/// 验证明文密码是否与已哈希的密码匹配（同步版本）。
///
/// 内部从哈希字符串中提取盐值，无需额外参数。
///
/// > **注意**：与 [`hash_passwd`] 同理，异步上下文中推荐使用 [`verify_passwd_async`]。
///
/// ## 返回值
///
/// - `Ok(true)`：密码匹配
/// - `Ok(false)`：密码不匹配
/// - `Err(_)`：哈希字符串格式错误等异常情况
pub fn verify_passwd(passwd: &str, hashed_passwd: &str) -> anyhow::Result<bool> {
    Ok(bcrypt::verify(passwd, hashed_passwd)?)
}

/// 对明文密码进行 bcrypt 哈希（异步版本）。
///
/// 通过 [`tokio::task::spawn_blocking`] 将 bcrypt 计算卸载到
/// 专用线程池，避免阻塞 tokio 异步运行时。
///
/// ## 使用示例
///
/// ```rust,ignore
/// async fn handler() -> anyhow::Result<()> {
///     let hashed = hash_passwd_async("mypassword".to_string()).await?;
///     Ok(())
/// }
/// ```
///
/// ## 注意事项
///
/// 参数为 `String` 而非 `&str`，因为 `spawn_blocking` 要求闭包是 `'static` 的，
/// 引用无法满足该约束。若已有 `&str`，调用 `.to_string()` 即可。
pub async fn hash_passwd_async(passwd: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || hash_passwd(&passwd)).await?
}

/// 验证明文密码是否与已哈希的密码匹配（异步版本）。
///
/// 同样通过 [`tokio::task::spawn_blocking`] 卸载计算，避免阻塞
/// tokio 异步运行时。
pub async fn verify_passwd_async(passwd: String, hashed_passwd: String) -> anyhow::Result<bool> {
    tokio::task::spawn_blocking(move || verify_passwd(&passwd, &hashed_passwd)).await?
}
