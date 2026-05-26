# daoyi-axum-app

应用启动与核心功能 crate，负责组装中间件链、数据库连接、JWT 认证等核心功能。

通过 `app::run()` 提供一键启动 HTTP 服务的入口函数，依赖 `daoyi-axum-config` 和 `daoyi-axum-support` 两个底层 crate。

## 核心模块

| 模块                   | 功能                                       |
|----------------------|------------------------------------------|
| `app::server`        | HTTP 服务器构建与启动（中间件链组装）                    |
| `app::database`      | 数据库连接池初始化（自适应 CPU 核心数）                   |
| `app::common`        | 通用数据结构（`PageParam`、`PageResult`）         |
| `app::auth`          | 认证主体 (`Principal`) 定义                    |
| `app::auth::jwt`     | JWT 编解码（HMAC-SHA256）+ Bearer Token 认证中间件 |
| `app::validation`    | 自定义参数校验函数（分页范围、手机号正则等）                   |
| `app::sea_orm_utils` | SeaORM 扩展工具函数（占位，可扩展）                    |

## 中间件链

请求按 middleware layer 顺序从外到内依次经过：

```
请求 → TimeoutLayer (120s)
     → DefaultBodyLimit (2 GiB)
     → TraceLayer (xid / IP / userId / 耗时)
     → CorsLayer (跨域)
     → NormalizePathLayer (路径规范化)
     → Router → JWT Auth Middleware → Handler
```

### 各中间件说明

| 中间件                  | 说明                      |
|----------------------|-------------------------|
| `TimeoutLayer`       | 120 秒超时保护，超时返回 408      |
| `DefaultBodyLimit`   | 请求体大小上限 2 GiB           |
| `TraceLayer`         | 结构化请求日志（含 xid、IP、耗时等字段） |
| `CorsLayer`          | 跨域配置（开发阶段允许所有来源）        |
| `NormalizePathLayer` | URL 路径规范化（去除尾部斜杠）       |

## 使用示例

```rust
use axum::{Router, routing::get};
use daoyi_axum_app::app::{self, AppState};

// 定义路由
async fn health_check() -> &'static str { "OK" }

let router = Router::new()
    .route("/health", get(health_check));

// 一键启动服务（自动完成日志、数据库、配置等初始化）
app::run(router).await?;
```

## `AppState`

全局应用状态，通过 Axum 的 `State` 提取器在处理器中共享：

```rust
use daoyi_axum_app::app::AppState;
use axum::extract::State;

async fn handler(state: State<AppState>) -> impl IntoResponse {
    let db = &state.db; // SeaORM 数据库连接池
    // ...
}
```

## JWT 认证

```rust
use daoyi_axum_app::app::auth::jwt::{JWT, Principal, JwtConfig};

// 创建 JWT 实例（使用默认配置：密钥 "12345678"，1 小时过期）
let jwt = JWT::default();

// 或自定义配置
let config = JwtConfig::new("my-secret-key", 3600);
let jwt = JWT::new(config);

// 编码 Token
let principal = Principal { tenant_id: 0, id: 1, name: "admin".into() };
let token = jwt.encode(principal)?;

// 解码 Token
let decoded = jwt.decode(&token)?;

// 使用全局默认 JWT 单例
use daoyi_axum_app::app::auth::jwt::default_jwt;
let token = default_jwt().encode(principal)?;

// 添加 Bearer Token 认证中间件
use daoyi_axum_app::app::auth::jwt::middleware::get_auth_layer;
let router = router.route_layer(get_auth_layer());
```

## 分页

```rust
use daoyi_axum_app::app::common::{PageParam, PageResult};

// PageParam 自动从全局配置读取默认值
// GET /users/page?pageNo=1&pageSize=20

// 构建分页结果
let result = PageResult::new(page_no, page_size, total_count, list);
// 或从 PageParam 直接构造
let result = PageResult::from_page_param(page_param, total_count, list);
```

## 数据库连接

连接池大小根据 CPU 核心数自动计算：

| 参数                | 计算公式               | 下限 | 说明    |
|-------------------|--------------------|----|-------|
| `min_connections` | `max(cpu * 4, 10)` | 10 | 最小连接数 |
| `max_connections` | `max(cpu * 8, 20)` | 20 | 最大连接数 |

## 依赖

| 依赖                    | 用途               |
|-----------------------|------------------|
| `daoyi-axum-config`   | 配置管理             |
| `daoyi-axum-support`  | 基础设施工具           |
| `axum` / `tower-http` | HTTP 框架与中间件      |
| `sea-orm`             | 异步 ORM           |
| `jsonwebtoken`        | JWT 编码 / 解码      |
| `num_cpus`            | CPU 核心数检测（连接池大小） |
| `xid`                 | 全局唯一请求 ID        |
| `bytesize`            | 人类可读字节大小         |
| `regex`               | 正则表达式（手机号校验）     |
| `validator`           | 声明式参数校验          |
