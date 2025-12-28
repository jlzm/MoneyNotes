use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::error::AppError;
use crate::utils::JwtUtil;

#[derive(Clone)]
pub struct AuthState {
    pub jwt_util: JwtUtil,
}

#[derive(Clone)]
pub struct CurrentUser {
    pub id: Uuid,
}

pub async fn auth_middleware(
    State(state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let user_id = state.jwt_util.extract_user_id(token)?;

    request.extensions_mut().insert(CurrentUser { id: user_id });

    Ok(next.run(request).await)
}

pub async fn optional_auth_middleware(
    State(state): State<AuthState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    if let Some(auth_header) = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
    {
        if let Some(token) = auth_header.strip_prefix("Bearer ") {
            if let Ok(user_id) = state.jwt_util.extract_user_id(token) {
                request.extensions_mut().insert(CurrentUser { id: user_id });
            }
        }
    }

    next.run(request).await
}
