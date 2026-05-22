//! 数据库初始化模块。
//!
//! 基于 SeaORM 建立数据库连接池，支持 MySQL、PostgreSQL、SQLite 三种数据库后端。
//! 连接池大小根据 CPU 核心数自动计算：最小 `cpu * 4`（下限 10），最大 `cpu * 8`（下限 20）。

use super::config;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use std::cmp::max;
use std::time::Duration;

/// 初始化数据库连接池。
///
/// 读取全局配置中的数据库连接信息，构建连接池并进行健康检查。
///
/// ## 连接池参数
///
/// | 参数 | 值 | 说明 |
/// |------|-----|------|
/// | `min_connections` | `max(cpu * 4, 10)` | 最小连接数 |
/// | `max_connections` | `max(cpu * 8, 20)` | 最大连接数 |
/// | `connect_timeout` | 10s | 建立连接超时 |
/// | `acquire_timeout` | 30s | 获取连接超时 |
/// | `idle_timeout` | 60s | 空闲连接回收时间 |
/// | `max_lifetime` | 300s | 连接最大存活时间 |
///
/// ## 错误处理
///
/// 连接失败或 ping 不通时返回 `anyhow::Error`。
pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = config::get().database();
    // 构建数据库连接 URL
    let mut options = ConnectOptions::new(format!(
        "{}://{}:{}@{}:{}/{}",
        database_config.protocol(),
        database_config.user(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database()
    ));

    let cpus = num_cpus::get() as u32;
    options
        .min_connections(max(cpus * 4, 10))
        .max_connections(max(cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(300))
        .sqlx_logging(false)
        .set_schema_search_path(database_config.schema());

    // 建立连接
    let db = Database::connect(options).await?;
    // 健康检查
    db.ping().await?;
    tracing::info!("{} 数据库连接成功", db.get_database_backend().as_str());
    // 打印数据库版本信息
    log_database_version(&db).await?;
    Ok(db)
}

/// 查询并打印数据库版本。
async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = get_database_version(db).await?;
    tracing::info!("数据库版本: {version}");
    Ok(())
}

/// 根据数据库后端类型执行相应的版本查询 SQL。
///
/// - MySQL: `SELECT VERSION()`  
/// - PostgreSQL: `SELECT version()`  
/// - SQLite: `SELECT sqlite_version()`
async fn get_database_version(db: &DatabaseConnection) -> anyhow::Result<String> {
    let db_backend = db.get_database_backend();

    // 根据数据库类型选择对应的 SQL 语句
    let sql = match db_backend {
        sea_orm::DbBackend::MySql => Ok("SELECT VERSION() as version"),
        sea_orm::DbBackend::Postgres => Ok("SELECT version() as version"),
        sea_orm::DbBackend::Sqlite => Ok("SELECT sqlite_version() as version"),
        _ => Err(DbErr::Custom("Unsupported database backend".to_owned())),
    }?;

    let stmt = Statement::from_string(db_backend, sql);
    let version = db
        .query_one_raw(stmt)
        .await?
        .ok_or_else(|| anyhow::anyhow!("获取数据库版本失败"))?
        .try_get::<String>("", "version")?;
    Ok(version)
}
