# daoyi-axum-support

基础设施支撑 crate，提供微服务开发中常用的工具模块，被 `daoyi-axum-app` 和 `daoyi-sea-orm-entity-demo` 共同依赖。

## 模块一览

| 模块                     | 功能            | 关键类型/函数                                              |
|------------------------|---------------|------------------------------------------------------|
| `support::error`       | 统一错误枚举        | `ApiError`、`ApiResult<T>`，自动映射 HTTP 状态码              |
| `support::response`    | 标准化 API 响应    | `ApiResponse<T>`（code/msg/data 结构）                   |
| `support::enumeration` | 通用枚举类型        | `Gender`（Male/Female/Unknown），实现 SeaORM ActiveEnum   |
| `support::id`          | 分布式 ID 生成     | `init()` / `next_id()` / `next_id_str()` — 基于雪花算法    |
| `support::passwd`      | 密码安全          | `hash_passwd()` / `verify_passwd()` — bcrypt 同步/异步版本 |
| `support::valid`       | 校验型提取器        | `ValidQuery<T>` / `ValidPath<T>` / `ValidJson<T>`    |
| `support::json`        | JSON Body 提取器 | 自定义 `Json<T>`，错误自动转换为 `ApiError`                     |
| `support::query`       | 查询参数提取器       | 自定义 `Query<T>`，错误自动转换为 `ApiError`                    |
| `support::path`        | 路径参数提取器       | 自定义 `Path<T>`，错误自动转换为 `ApiError`                     |
| `support::serde`       | 反序列化工具        | `deserialize_number` — 字符串/数字兼容的数字反序列化               |
| `support::logger`      | 结构化日志         | `logger::init()` — tracing-subscriber 初始化（本地时间+时区）   |
| `support::latency`     | 请求耗时记录        | `LatencyOnResponse` — TraceLayer 响应回调                |

## 使用示例

```rust
use daoyi_axum_support::support;

// 初始化日志
support::logger::init();

// 生成分布式 ID
support::id::init()?;
let id: i64 = support::id::next_id();

// 密码哈希与验证
let hashed = support::passwd::hash_passwd("mypassword")?;
let valid = support::passwd::verify_passwd("mypassword", &hashed)?;

// 异步版密码验证（推荐用于 axum handler）
let ok = support::passwd::verify_passwd_async(
    "password".to_string(),
    hashed.to_string()
).await?;

// 统一错误处理
use support::error::{ApiError, ApiResult};
fn do_something() -> ApiResult<String> {
    Err(ApiError::NotFound("用户不存在".into()))
}

// 统一响应格式
use support::response::{success, fail};
let resp = success(data);
let err = fail("操作失败");
```

## 依赖

| 依赖                               | 用途                    |
|----------------------------------|-----------------------|
| `sea-orm`                        | SeaORM 核心类型           |
| `axum` / `tower-http`            | HTTP 工具类型             |
| `tracing` / `tracing-subscriber` | 结构化日志                 |
| `axum-valid` / `validator`       | 请求参数校验                |
| `idgenerator`                    | 雪花算法 ID 生成            |
| `bcrypt`                         | 密码哈希                  |
| `serde`                          | 序列化 / 反序列化            |
| `thiserror` / `anyhow`           | 错误处理                  |
| `jsonwebtoken`                   | JWT 错误类型（跨 crate 复用）  |
| `tokio`                          | 异步运行时（spawn_blocking） |
