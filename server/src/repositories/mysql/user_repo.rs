use async_trait::async_trait;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateUser, UpdateUser, User};
use crate::repositories::traits::UserRepository;

pub struct MySqlUserRepository {
    pool: MySqlPool,
}

impl MySqlUserRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for MySqlUserRepository {
    async fn create(&self, user: CreateUser) -> AppResult<User> {
        let new_user = User::new(user.email, user.password_hash, user.nickname);

        sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, nickname, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_user.id.to_string())
        .bind(&new_user.email)
        .bind(&new_user.password_hash)
        .bind(&new_user.nickname)
        .bind(new_user.created_at)
        .bind(new_user.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(new_user)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            r#"
            SELECT id, email, password_hash, nickname, avatar, created_at, updated_at
            FROM users WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|(id, email, password_hash, nickname, avatar, created_at, updated_at)| User {
            id: Uuid::parse_str(&id).unwrap(),
            email,
            password_hash,
            nickname,
            avatar,
            created_at,
            updated_at,
        }))
    }

    async fn find_by_email(&self, email: &str) -> AppResult<Option<User>> {
        let result = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, chrono::DateTime<chrono::Utc>, chrono::DateTime<chrono::Utc>)>(
            r#"
            SELECT id, email, password_hash, nickname, avatar, created_at, updated_at
            FROM users WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|(id, email, password_hash, nickname, avatar, created_at, updated_at)| User {
            id: Uuid::parse_str(&id).unwrap(),
            email,
            password_hash,
            nickname,
            avatar,
            created_at,
            updated_at,
        }))
    }

    async fn update(&self, id: Uuid, user: UpdateUser) -> AppResult<User> {
        sqlx::query(
            r#"
            UPDATE users SET nickname = COALESCE(?, nickname), avatar = COALESCE(?, avatar), updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(&user.nickname)
        .bind(&user.avatar)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("User not found".to_string()))
    }

    async fn update_password(&self, id: Uuid, password_hash: String) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE users SET password_hash = ?, updated_at = NOW() WHERE id = ?
            "#,
        )
        .bind(&password_hash)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}
