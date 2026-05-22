# web-starter

daoyi-cloud-axum 的 Axum Web 服务启动示例。

完整演示 Axum + SeaORM + Tracing 微服务技术栈的最佳实践，包含路由管理、参数校验、数据库 CRUD、统一错误处理、分布式 ID
生成、密码安全、结构化日志等功能。

## 功能

- **HTTP 服务器**：基于 `axum` 0.8，支持中间件链（CORS、超时、Body 限制、路径规范化）
- **配置管理**：通过 `config` crate 读取 YAML 配置文件，支持命令行参数与环境变量覆盖
- **数据库 ORM**：基于 `sea-orm` 2.0 异步 ORM，连接池大小自适应 CPU 核心数
- **参数校验**：集成 `validator` + `axum-valid`，支持查询参数、路径参数、JSON Body 自动校验
- **统一错误处理**：错误自动映射为 HTTP 状态码与 JSON 响应
- **分布式 ID**：基于 `idgenerator` 的雪花算法全局唯一 ID 生成
- **密码安全**：基于 `bcrypt` 的密码哈希与验证
- **分页支持**：内置通用分页参数与分页响应结构体
- **结构化日志**：基于 `tracing`，记录请求 ID、客户端 IP、响应耗时，支持本地时间 + 时区偏移
- **SeaORM Entity**：包含 6 张 demo 表的自动生成 Entity 模型

## 运行

```bash
# 在项目根目录运行
cargo run -p web-starter

# 指定应用名称与配置文件前缀
APP_NAME=example-web-starter cargo run -p web-starter

# 指定端口（通过环境变量覆盖）
APP_SERVER_PORT=8080 cargo run -p web-starter

# 指定配置文件
cargo run -p web-starter -- -c resources/example-web-starter-dev.yaml
```

## API 端点

| 方法     | 路径                                              | 说明                    |
|--------|-------------------------------------------------|-----------------------|
| GET    | `/`                                             | 欢迎页                   |
| GET    | `/api/users`                                    | 条件查询用户列表（固定条件演示）      |
| GET    | `/api/users/page?keyword=&pageNo=1&pageSize=10` | 分页查询（支持 keyword 模糊搜索） |
| POST   | `/api/users`                                    | 创建用户                  |
| PUT    | `/api/users/{id}`                               | 更新用户                  |
| DELETE | `/api/users/{id}`                               | 删除用户                  |

### 响应示例

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "pageNo": 1,
    "pageSize": 10,
    "totalPage": 5,
    "total": 42,
    "list": [
      ...
    ]
  }
}
```

## 配置

默认配置文件路径规则：`resources/{APP_NAME}-{APP_PROFILE}.yaml`

示例配置文件 `resources/example-web-starter-dev.yaml`：

```yaml
server:
  port: 3001

database:
  host: 127.0.0.1
  port: 3306
  user: root
  password: 123456
  database: demo

sys:
  page_size_min: 10      # 分页最小条数
  page_size_max: 100     # 分页最大条数
  page_size_default: 20  # 分页默认条数
  page_no_default: 1     # 默认页码
```

## 模块

| 模块                 | 说明                                             |
|--------------------|------------------------------------------------|
| `main`             | 服务入口，模块声明与路由传递                                 |
| `app`              | 应用启动流程与全局 State 定义                             |
| `api`              | API 路由组装（根路径 + 子路由 + fallback）                 |
| `api::user`        | 用户 API 处理器（完整 CRUD：查询 + 分页 + 创建 + 更新 + 删除）     |
| `common`           | 通用数据结构（`PageParam`、`PageResult`）               |
| `config`           | YAML 配置加载（支持环境变量与命令行覆盖）                        |
| `config::server`   | 服务器端口配置                                        |
| `config::database` | 数据库连接配置                                        |
| `config::sys`      | 系统通用配置（分页限制等）                                  |
| `database`         | 数据库连接池初始化（自适应 CPU 核心数）                         |
| `demo::entity`     | SeaORM Entity 模型（6 张 demo 表，自动生成）              |
| `enumeration`      | 枚举类型定义（`Gender` 等）                             |
| `error`            | 统一错误枚举与 HTTP 状态码映射                             |
| `id`               | 分布式 ID 生成器（雪花算法）                               |
| `json`             | 自定义 JSON 提取器（错误自动映射）                           |
| `latency`          | TraceLayer 响应耗时记录回调                            |
| `logger`           | tracing 日志订阅器初始化                               |
| `passwd`           | 密码哈希与验证（bcrypt）                                |
| `path`             | 自定义路径参数提取器（错误自动映射）                             |
| `query`            | 自定义查询参数提取器（错误自动映射）                             |
| `response`         | 统一 API 响应格式（`ApiResponse`）                     |
| `sea_orm_utils`    | SeaORM 扩展工具函数（占位）                              |
| `serde`            | 自定义反序列化函数（字符串/数字兼容）                            |
| `server`           | HTTP 服务器构建（中间件链组装）                             |
| `valid`            | 校验型参数提取器（`ValidQuery`/`ValidPath`/`ValidJson`） |
| `validation`       | 自定义校验函数（分页范围、手机号正则）                            |

## 开发指南

### 生成 SeaORM Entity

```shell
cargo install sea-orm-cli@^2.0.0-rc

sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./example/web-starter/src/demo/entity
```

### 中间件链顺序

中间件按定义的 layer 顺序从外到内执行：

```
请求 → TimeoutLayer (120s) → DefaultBodyLimit (2GiB)
     → TraceLayer (日志/追踪/耗时) → CorsLayer (跨域)
     → NormalizePathLayer (去尾部斜杠) → Router → Handler
```