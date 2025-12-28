use std::sync::Arc;

use crate::error::{AppError, AppResult};
use crate::models::{CreateUser, User};
use crate::repositories::UserRepository;
use crate::utils::{hash_password, verify_password, JwtUtil};

pub struct AuthService {
    user_repo: Arc<dyn UserRepository>,
    jwt_util: JwtUtil,
}

impl AuthService {
    pub fn new(user_repo: Arc<dyn UserRepository>, jwt_util: JwtUtil) -> Self {
        Self { user_repo, jwt_util }
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
        nickname: Option<String>,
    ) -> AppResult<(User, String, String)> {
        // Check if user exists
        if self.user_repo.find_by_email(&email).await?.is_some() {
            return Err(AppError::Conflict("User already exists".to_string()));
        }

        // Hash password
        let password_hash = hash_password(&password)?;

        // Create user
        let user = self
            .user_repo
            .create(CreateUser {
                email,
                password_hash,
                nickname,
            })
            .await?;

        // Generate tokens
        let access_token = self.jwt_util.generate_access_token(user.id)?;
        let refresh_token = self.jwt_util.generate_refresh_token(user.id)?;

        Ok((user, access_token, refresh_token))
    }

    pub async fn login(&self, email: String, password: String) -> AppResult<(User, String, String)> {
        // Find user
        let user = self
            .user_repo
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        // Verify password
        if !verify_password(&password, &user.password_hash)? {
            return Err(AppError::Unauthorized);
        }

        // Generate tokens
        let access_token = self.jwt_util.generate_access_token(user.id)?;
        let refresh_token = self.jwt_util.generate_refresh_token(user.id)?;

        Ok((user, access_token, refresh_token))
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> AppResult<String> {
        let user_id = self.jwt_util.extract_user_id(refresh_token)?;

        // Verify user still exists
        self.user_repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::Unauthorized)?;

        // Generate new access token
        self.jwt_util.generate_access_token(user_id)
    }
}
