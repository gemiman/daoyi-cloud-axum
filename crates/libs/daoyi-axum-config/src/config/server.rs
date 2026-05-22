//! 服务器配置子模块。
//!
//! 提供 HTTP 服务器监听端口等运行时配置。

use serde::Deserialize;

/// 服务器相关配置。
///
/// 对应 YAML 配置文件中 `server` 节点，可通过 `APP_SERVER_PORT`
/// 环境变量进行覆盖。
///
/// ## 示例 YAML
///
/// ```yaml
/// server:
///   port: 3001
/// ```
///
/// ## 配置项
///
/// | 字段 | 类型 | 默认值 | 说明 |
/// |------|------|--------|------|
/// | `port` | `u16` | `3000` | HTTP 监听端口 |
#[derive(Debug, Deserialize, Default)]
pub struct ServerConfig {
    /// 监听端口，未配置时默认使用 `3000`。
    port: Option<u16>,
}

impl ServerConfig {
    /// 获取服务器监听端口。
    ///
    /// 若配置文件中未指定 `port`，则返回默认值 `3000`。
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
