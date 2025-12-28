use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateUser, UpdateUser, User};
use crate::repositories::traits::UserRepository;

pub struct MemoryUserRepository {
    users: RwLock<HashMap<Uuid, User>>,
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for MemoryUserRepository {
    async fn create(&self, user: CreateUser) -> AppResult<User> {
        let new_user = User::new(user.email, user.password_hash, user.nickname);
        let mut users = self.users.write().unwrap();
        users.insert(new_user.id, new_user.clone());
        Ok(new_user)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let users = self.users.read().unwrap();
        Ok(users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let users = self.users.read().unwrap();
        Ok(users.values().find(|u| u.email == email).cloned())
    }

    async fn update(&self, id: Uuid, update: UpdateUser) -> AppResult<User> {
        let mut users = self.users.write().unwrap();
        let user = users.get_mut(&id).ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

        if let Some(nickname) = update.nickname {
            user.nickname = Some(nickname);
        }
        if let Some(avatar) = update.avatar {
            user.avatar = Some(avatar);
        }
        user.updated_at = chrono::Utc::now();

        Ok(user.clone())
    }

    async fn update_password(&self, id: Uuid, password_hash: String) -> AppResult<()> {
        let mut users = self.users.write().unwrap();
        let user = users.get_mut(&id).ok_or_else(|| AppError::NotFound("User not found".to_string()))?;
        user.password_hash = password_hash;
        user.updated_at = chrono::Utc::now();
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut users = self.users.write().unwrap();
        users.remove(&id);
        Ok(())
    }
}
