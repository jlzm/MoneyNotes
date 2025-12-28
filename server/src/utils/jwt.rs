use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub exp: i64,     // expiration time
    pub iat: i64,     // issued at
}

#[derive(Clone)]
pub struct JwtUtil {
    secret: String,
    access_expires: i64,
    refresh_expires: i64,
}

impl JwtUtil {
    pub fn new(secret: String, access_expires: i64, refresh_expires: i64) -> Self {
        Self {
            secret,
            access_expires,
            refresh_expires,
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(self.access_expires)).timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> AppResult<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id.to_string(),
            exp: (now + Duration::seconds(self.refresh_expires)).timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    pub fn verify_token(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Jwt(e.to_string()))
    }

    pub fn extract_user_id(&self, token: &str) -> AppResult<Uuid> {
        let claims = self.verify_token(token)?;
        Uuid::parse_str(&claims.sub).map_err(|e| AppError::Jwt(format!("Invalid user id: {}", e)))
    }
}
