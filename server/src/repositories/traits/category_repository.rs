use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{BillType, Category, CreateCategory};

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, category: CreateCategory) -> AppResult<Category>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Category>>;
    async fn find_by_ledger_id(&self, ledger_id: Option<Uuid>, category_type: Option<BillType>) -> AppResult<Vec<Category>>;
    async fn update(&self, id: Uuid, name: Option<String>, icon: Option<String>, sort_order: Option<i32>) -> AppResult<Category>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;

    async fn init_default_categories(&self) -> AppResult<()>;
}
