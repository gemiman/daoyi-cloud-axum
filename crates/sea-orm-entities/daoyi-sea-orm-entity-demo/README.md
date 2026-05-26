# daoyi-sea-orm-entity-demo

数据库 Entity 模型演示 crate，包含由 `sea-orm-cli generate` 自动生成的 ORM 实体模型。

对应 `demo` 数据库中的所有表，每个实体文件包含对应的 Model、Relation 枚举和 ActiveModelBehavior 实现。

## 表一览

| 实体模块                          | 数据库表名           | 说明                             |
|-------------------------------|-----------------|--------------------------------|
| `demo::entity::demo_sys_user` | `demo_sys_user` | 系统用户表（含账号、密码、手机号、生日等完整字段）      |
| `demo::entity::demo_category` | `demo_category` | 分类表（支持父子层级，通过 `parent_id` 自引用） |
| `demo::entity::demo_contact`  | `demo_contact`  | 联系人表（姓名、性别、生日、描述、头像）           |
| `demo::entity::demo_course`   | `demo_course`   | 课程表（关联 `student_id`，课程名、分数）    |
| `demo::entity::demo_grade`    | `demo_grade`    | 成绩表（关联 `student_id`，科目、教师）     |
| `demo::entity::demo_student`  | `demo_student`  | 学生表（姓名、性别、生日、描述）               |

### 公共字段

所有表均包含以下标准审计字段和租户隔离字段：

| 字段            | 类型         | 说明           |
|---------------|------------|--------------|
| `creator`     | `String`   | 创建人          |
| `create_time` | `DateTime` | 创建时间         |
| `updater`     | `String`   | 更新人          |
| `update_time` | `DateTime` | 更新时间         |
| `deleted`     | `bool`     | 逻辑删除标记       |
| `tenant_id`   | `i64`      | 租户 ID（多租户隔离） |

## 使用示例

```rust
use daoyi_sea_orm_entity_demo::demo::entity::demo_sys_user;
use daoyi_sea_orm_entity_demo::demo::entity::prelude::*;
use sea_orm::*;

// 查询用户
let user = DemoSysUser::find()
    .filter(demo_sys_user::Column::Account.eq("admin"))
    .one(db)
    .await?;

// 创建用户
let new_user = demo_sys_user::ActiveModel {
    name: Set("张三".into()),
    account: Set("zhangsan".into()),
    password: Set("hashed_password".into()),
    ..Default::default()
};

// 插入
let result = new_user.insert(db).await?;

// 更新用户
let mut active: demo_sys_user::ActiveModel = user.unwrap().into();
active.name = Set("新名字".into());
active.update(db).await?;

// 删除用户
demo_sys_user::Entity::delete_by_id(id).exec(db).await?;
```

## 生成 Entity

当数据库表结构变化时，使用以下命令重新生成 Entity：

```bash
# 安装 CLI 工具
cargo install sea-orm-cli@^2.0.0-rc

# 生成 Entity（需在项目根目录执行）
sea-orm-cli generate entity \
  -u mysql://root:123456@127.0.0.1:3306/demo \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --date-time-crate chrono \
  -o ./crates/sea-orm-entities/daoyi-sea-orm-entity-demo/src/demo/entity
```

> **注意**：生成后可能需要手动将 `gender` 字段类型改为自定义 `Gender` 枚举（参考 `demo_sys_user.rs` 中的 `Gender` 用法）。

## 依赖

| 依赖                   | 用途            |
|----------------------|---------------|
| `daoyi-axum-support` | 基础设施支撑（枚举类型等） |
| `sea-orm`            | ORM 框架        |
| `serde`              | 序列化 / 反序列化    |
