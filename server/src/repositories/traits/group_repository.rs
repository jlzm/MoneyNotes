use async_trait::async_trait;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{CreateGroup, Group, GroupMember, GroupRole};

#[async_trait]
pub trait GroupRepository: Send + Sync {
    async fn create(&self, group: CreateGroup) -> AppResult<Group>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Group>>;
    async fn find_by_invite_code(&self, code: &str) -> AppResult<Option<Group>>;
    async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Group>>;
    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>) -> AppResult<Group>;
    async fn update_invite_code(&self, id: Uuid, new_code: String) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn transfer_ownership(&self, id: Uuid, new_owner_id: Uuid) -> AppResult<()>;

    // Member operations
    async fn add_member(&self, group_id: Uuid, user_id: Uuid, role: GroupRole) -> AppResult<GroupMember>;
    async fn remove_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<()>;
    async fn update_member_role(&self, group_id: Uuid, user_id: Uuid, role: GroupRole) -> AppResult<()>;
    async fn get_members(&self, group_id: Uuid) -> AppResult<Vec<GroupMember>>;
    async fn get_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<Option<GroupMember>>;
}
