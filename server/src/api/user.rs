use axum::{
    extract::State,
    Extension, Json,
};
use std::sync::Arc;

use crate::dto::{
    ApiResponse, ChangePasswordRequest, UpdateUserRequest, UserResponse,
};
use crate::error::{AppError, AppResult};
use crate::middleware::CurrentUser;
use crate::models::UpdateUser;
use crate::repositories::UserRepository;
use crate::utils::{hash_password, verify_password};

pub struct UserApi {
    user_repo: Arc<dyn UserRepository>,
}

impl UserApi {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

pub async fn get_me(
    State(api): State<Arc<UserApi>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user = api
        .user_repo
        .find_by_id(current_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(ApiResponse::success(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        nickname: user.nickname,
        avatar: user.avatar,
        created_at: user.created_at.to_rfc3339(),
    })))
}

pub async fn update_me(
    State(api): State<Arc<UserApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user = api
        .user_repo
        .update(
            current_user.id,
            UpdateUser {
                nickname: req.nickname,
                avatar: req.avatar,
            },
        )
        .await?;

    Ok(Json(ApiResponse::success(UserResponse {
        id: user.id.to_string(),
        email: user.email,
        nickname: user.nickname,
        avatar: user.avatar,
        created_at: user.created_at.to_rfc3339(),
    })))
}

pub async fn change_password(
    State(api): State<Arc<UserApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<ChangePasswordRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    let user = api
        .user_repo
        .find_by_id(current_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    // Verify old password
    if !verify_password(&req.old_password, &user.password_hash)? {
        return Err(AppError::Validation("Invalid old password".to_string()));
    }

    // Hash new password
    let new_hash = hash_password(&req.new_password)?;

    api.user_repo.update_password(current_user.id, new_hash).await?;

    Ok(Json(ApiResponse::success(())))
}
