# daoyi-sea-orm-entity-demo

数据库 Entity 模型演示 crate，包含由 `sea-orm-cli generate` 自动生成的 ORM 实体模型。

对应 `demo` 数据库中的所有表，每个实体文件包含对应的 Model、Relation 枚举和 ActiveModelBehavior 实现。

## 表一览

| 实体模块                          | 数据库表名           | 说明                     |
|-------------------------------|-----------------|------------------------|
| `demo::entity::demo_sys_user` | `demo_sys_user` | 系统用户表（含账号、密码、手机号等完整字段） |
| `demo::entity::demo_category` | `demo_category` | 分类表（支持父子层级）            |
| `demo::entity::demo_contact`  | `demo_contact`  | 联系人表                   |
| `demo::entity::demo_course`   | `demo_course`   | 课程表                    |
| `demo::entity::demo_grade`    | `demo_grade`    | 成绩表                    |
| `demo::entity::demo_student`  | `demo_student`  | 学生表                    |

## 使用示例

```rust
use daoyi_sea_orm_entity_demo::demo::entity::demo_sys_user;
use sea_orm::*;

// 查询用户
let user = demo_sys_user::Entity::find()
    .filter(demo_sys_user::Column::Account.eq("admin"))
    .one(db)
    .await?;

// 创建用户
let new_user = demo_sys_user::ActiveModel {
    name: Set("张三".into()),
    account: Set("zhangsan".into()),
    ..Default::default()
};

// 插入
let result = new_user.insert(db).await?;
```

## 生成 Entity

当数据库表结构变化时，使用以下命令重新生成 Entity：

```bash
sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./crates/sea-orm-entities/daoyi-sea-orm-entity-demo/src/demo/entity
```

## 依赖

| 依赖                   | 用途       |
|----------------------|----------|
| `daoyi-axum-support` | 基础设施支撑   |
| `sea-orm`            | ORM 框架   |
| `serde`              | 序列化/反序列化 |
