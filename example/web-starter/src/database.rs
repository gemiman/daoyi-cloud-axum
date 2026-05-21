use super::config;
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};
use std::cmp::max;
use std::time::Duration;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config = config::get().database();
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
    let db = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!("数据库连接成功");
    log_database_version(&db).await?;
    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = get_database_version(db).await?;
    tracing::info!("数据库版本: {version}");
    Ok(())
}

async fn get_database_version(db: &DatabaseConnection) -> anyhow::Result<String> {
    let db_backend = db.get_database_backend();

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
