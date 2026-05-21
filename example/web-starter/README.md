# web-starter

daoyi-cloud-axum 的 Axum Web 服务启动示例。

## 功能

- 基于 `axum` 的 HTTP 服务器
- 通过 `config` crate 读取 YAML 配置文件
- 支持命令行参数与环境变量覆盖配置
- 基于 `tracing` + `tracing-subscriber` 的结构化日志

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

## 配置

默认配置文件路径规则：`resources/{APP_NAME}-{APP_PROFILE}.yaml`

示例配置文件 `resources/example-web-starter-dev.yaml`：

```yaml
server:
  port: 3001
```

## 模块

| 模块       | 说明               |
|----------|------------------|
| `main`   | 服务入口，路由注册与启动     |
| `logger` | tracing 日志订阅器初始化 |
| `config` | YAML 配置加载与反序列化   |

## 开发指南

### 生成 SeaORM Entity

```shell
cargo install sea-orm-cli@^2.0.0-rc
cd crates/libs/entities/daoyi-entity-demo
sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./src/demo/entity
```