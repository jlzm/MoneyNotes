use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateLedger, Ledger};

#[async_trait]
pub trait LedgerRepository: Send + Sync {
    async fn create(&self, ledger: CreateLedger) -> AppResult<Ledger>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Ledger>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Ledger>>;
    async fn find_by_group_id(&self, group_id: Uuid) -> AppResult<Vec<Ledger>>;
    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>) -> AppResult<Ledger>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
