# daoyi-axum-support

基础设施支撑 crate，提供微服务开发中常用的工具模块，被 `daoyi-axum-app` 和 `daoyi-sea-orm-entity-demo` 共同依赖。

## 模块一览

| 模块                     | 功能            | 关键类型/函数                                           |
|------------------------|---------------|---------------------------------------------------|
| `support::error`       | 统一错误枚举        | `AppError`、`AppResult<T>`，自动映射 HTTP 状态码           |
| `support::response`    | 标准化 API 响应    | `ApiResponse<T>`（code/msg/data 结构）                |
| `support::enumeration` | 通用枚举类型        | 性别枚举等                                             |
| `support::id`          | 分布式 ID 生成     | `id::generate_id()` — 基于雪花算法                      |
| `support::passwd`      | 密码安全          | `hash()` / `verify()` — bcrypt 哈希与验证              |
| `support::valid`       | 校验型提取器        | `ValidQuery<T>` / `ValidPath<T>` / `ValidJson<T>` |
| `support::json`        | JSON Body 提取器 | 自定义 `Json<T>`，错误自动转换为 HTTP 响应                     |
| `support::query`       | 查询参数提取器       | 自定义 `Query<T>`，错误自动转换                             |
| `support::path`        | 路径参数提取器       | 自定义 `Path<T>`，错误自动转换                              |
| `support::serde`       | 反序列化工具        | 字符串/数字兼容的反序列化函数                                   |
| `support::logger`      | 结构化日志         | `logger::init()` — tracing-subscriber 初始化         |
| `support::latency`     | 请求耗时记录        | `TraceLayer` 回调，记录每个请求的响应时间                       |

## 使用示例

```rust
use daoyi_axum_support::support;

// 初始化日志
support::logger::init();

// 生成分布式 ID
let id: i64 = support::id::generate_id();

// 密码哈希与验证
let hashed = support::passwd::hash("mypassword")?;
let valid = support::passwd::verify("mypassword", &hashed)?;

// 统一错误处理
use support::error::{AppError, AppResult};
fn do_something() -> AppResult<String> {
    Err(AppError::NotFound("用户不存在".into()))
}

// 统一响应格式
use support::response::ApiResponse;
let resp = ApiResponse::success(data);
```

## 依赖

| 依赖                               | 用途                |
|----------------------------------|-------------------|
| `sea-orm`                        | SeaORM 核心类型（后续扩展） |
| `axum` / `tower-http`            | HTTP 工具类型         |
| `tracing` / `tracing-subscriber` | 结构化日志             |
| `axum-valid` / `validator`       | 请求参数校验            |
| `idgenerator`                    | 雪花算法 ID 生成        |
| `bcrypt`                         | 密码哈希              |
| `serde`                          | 序列化/反序列化          |
| `thiserror` / `anyhow`           | 错误处理              |
