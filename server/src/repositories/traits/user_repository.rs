use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateUser, UpdateUser, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: CreateUser) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>>;
    async fn update(&self, id: Uuid, user: UpdateUser) -> AppResult<User>;
    async fn update_password(&self, id: Uuid, password_hash: String) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
