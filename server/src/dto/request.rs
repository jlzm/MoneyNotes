use serde::{Deserialize, Serialize};
use validator::Validate;

// Auth DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub password: String,
    #[validate(length(max = 50, message = "Nickname too long"))]
    pub nickname: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

// User DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(max = 50, message = "Nickname too long"))]
    pub nickname: Option<String>,
    #[validate(url(message = "Invalid avatar URL"))]
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters"))]
    pub new_password: String,
}

// Ledger DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateLedgerRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
    pub description: Option<String>,
    pub currency: Option<String>,
}

// Bill DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBillRequest {
    pub ledger_id: String,
    pub category_id: String,
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: f64,
    #[serde(rename = "type")]
    pub bill_type: String,  // "income" or "expense"
    pub note: Option<String>,
    pub bill_date: String,  // YYYY-MM-DD
}

#[derive(Debug, Deserialize)]
pub struct UpdateBillRequest {
    pub category_id: Option<String>,
    pub amount: Option<f64>,
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    pub note: Option<String>,
    pub bill_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BillQueryParams {
    pub ledger_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    pub category_id: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

// Group DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateGroupRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JoinGroupRequest {
    pub invite_code: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMemberRoleRequest {
    pub role: String,  // "admin" or "member"
}

#[derive(Debug, Deserialize)]
pub struct TransferGroupRequest {
    pub new_owner_id: String,
}

// Category DTOs
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 50, message = "Name must be 1-50 characters"))]
    pub name: String,
    pub icon: Option<String>,
    #[serde(rename = "type")]
    pub category_type: String,  // "income" or "expense"
    pub parent_id: Option<String>,
}
