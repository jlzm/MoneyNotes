use axum::{extract::State, Json};
use std::sync::Arc;
use validator::Validate;

use crate::dto::{
    ApiResponse, AuthResponse, LoginRequest, RefreshTokenRequest, RefreshTokenResponse,
    RegisterRequest, UserResponse,
};
use crate::error::{AppError, AppResult};
use crate::services::AuthService;

pub struct AuthApi {
    auth_service: Arc<AuthService>,
    expires_in: i64,
}

impl AuthApi {
    pub fn new(auth_service: Arc<AuthService>, expires_in: i64) -> Self {
        Self {
            auth_service,
            expires_in,
        }
    }
}

pub async fn register(
    State(api): State<Arc<AuthApi>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<ApiResponse<AuthResponse>>> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let (user, access_token, refresh_token) = api
        .auth_service
        .register(req.email, req.password, req.nickname)
        .await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            nickname: user.nickname,
            avatar: user.avatar,
            created_at: user.created_at.to_rfc3339(),
        },
        access_token,
        refresh_token,
        expires_in: api.expires_in,
    })))
}

pub async fn login(
    State(api): State<Arc<AuthApi>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<AuthResponse>>> {
    req.validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let (user, access_token, refresh_token) = api
        .auth_service
        .login(req.email, req.password)
        .await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        user: UserResponse {
            id: user.id.to_string(),
            email: user.email,
            nickname: user.nickname,
            avatar: user.avatar,
            created_at: user.created_at.to_rfc3339(),
        },
        access_token,
        refresh_token,
        expires_in: api.expires_in,
    })))
}

pub async fn refresh(
    State(api): State<Arc<AuthApi>>,
    Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<ApiResponse<RefreshTokenResponse>>> {
    let access_token = api.auth_service.refresh_token(&req.refresh_token).await?;

    Ok(Json(ApiResponse::success(RefreshTokenResponse {
        access_token,
        expires_in: api.expires_in,
    })))
}
