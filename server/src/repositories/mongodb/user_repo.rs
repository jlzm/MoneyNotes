use async_trait::async_trait;
use mongodb::{bson::doc, Collection, Database};
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateUser, UpdateUser, User};
use crate::repositories::traits::UserRepository;

pub struct MongoUserRepository {
    collection: Collection<User>,
}

impl MongoUserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }
}

#[async_trait]
impl UserRepository for MongoUserRepository {
    async fn create(&self, user: CreateUser) -> AppResult<User> {
        let new_user = User::new(user.email, user.password_hash, user.nickname);

        self.collection
            .insert_one(&new_user, None)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(new_user)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        self.collection
            .find_one(doc! { "id": id.to_string() }, None)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        self.collection
            .find_one(doc! { "email": email }, None)
            .await
            .map_err(|e| AppError::Database(e.to_string()))
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> AppResult<User> {
        let mut update_doc = doc! {};
        if let Some(nickname) = &user.nickname {
            update_doc.insert("nickname", nickname);
        }
        if let Some(avatar) = &user.avatar {
            update_doc.insert("avatar", avatar);
        }
        update_doc.insert("updated_at", chrono::Utc::now());

        self.collection
            .update_one(doc! { "id": id.to_string() }, doc! { "$set": update_doc }, None)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    async fn update_password(&self, id: Uuid, password_hash: String) -> AppResult<()> {
        self.collection
            .update_one(
                doc! { "id": id.to_string() },
                doc! { "$set": { "password_hash": password_hash, "updated_at": chrono::Utc::now() } },
                None,
            )
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        self.collection
            .delete_one(doc! { "id": id.to_string() }, None)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
