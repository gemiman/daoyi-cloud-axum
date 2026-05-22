//! 数据库配置子模块。
//!
//! 提供 MySQL/PostgreSQL 连接所需的所有配置项，所有字段均为可选，
//! 未配置时使用内置默认值。

use serde::Deserialize;

/// 数据库连接配置。
///
/// 对应 YAML 配置文件中 `database` 节点，可通过 `APP_DATABASE_HOST`、
/// `APP_DATABASE_PORT` 等环境变量进行覆盖。
///
/// ## 示例 YAML
///
/// ```yaml
/// database:
///   protocol: mysql
///   host: 127.0.0.1
///   port: 3306
///   user: root
///   password: 123456
///   database: demo
/// ```
#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库协议，默认 `"mysql"`。
    protocol: Option<String>,

    /// 数据库主机地址，默认 `"127.0.0.1"`。
    host: Option<String>,

    /// 数据库端口，默认 `3306`。
    port: Option<u16>,

    /// 数据库用户名，默认 `"root"`。
    user: Option<String>,

    /// 数据库密码，默认 `"123456"`。
    password: Option<String>,

    /// 数据库名称，默认 `"demo"`。
    database: Option<String>,

    /// 数据库 schema/搜索路径，默认为空字符串。
    schema: Option<String>,
}

impl DatabaseConfig {
    /// 获取数据库协议。
    pub fn protocol(&self) -> &str {
        self.protocol.as_deref().unwrap_or("mysql")
    }

    /// 获取数据库主机地址。
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    /// 获取数据库端口。
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3306)
    }

    /// 获取数据库用户名。
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("root")
    }

    /// 获取数据库密码。
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("123456")
    }

    /// 获取数据库名称。
    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("demo")
    }

    /// 获取数据库 schema / 搜索路径。
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("")
    }
}
