//! SeaORM 工具函数模块。
//!
//! 提供与 SeaORM ORM 相关的通用辅助函数和扩展 trait。
//!
//! ## 设计初衷
//!
//! 此模块是扩展功能的预留位置。随着项目演进，可以在此添加：
//!
//! - **自定义查询构造器**：封装常用的链式查询逻辑（如通用的分页查询、多条件过滤）
//! - **数据转换工具**：Model ↔ DTO 的便捷转换函数
//! - **事务辅助**：简化事务管理的包装函数
//! - **迁移工具**：数据库迁移的便捷调用
//! - **审计钩子**：在 ActiveModelBehavior 中自动填充 `creator`/`updater` 等审计字段
//!
//! ## 使用建议
//!
//! 建议将跨模块复用的 SeaORM 操作提取到此模块中，
//! 避免在 Handler 中编写重复的数据库查询逻辑。
//!
//! ## 示例（未来可参考）
//!
//! ```rust,ignore
//! use sea_orm::PaginatorTrait;
//!
//! /// 通用的分页查询辅助函数
//! pub async fn find_page<E: EntityTrait>(
//!     db: &DatabaseConnection,
//!     page_no: u64,
//!     page_size: u64,
//! ) -> Result<PageResult<E::Model>, DbErr> {
//!     // ...
//! }
//! ```
