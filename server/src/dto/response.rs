use serde::Serialize;

// Generic response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_message(message: &str) -> ApiResponse<()> {
        ApiResponse {
            code: 0,
            message: message.to_string(),
            data: None,
        }
    }
}

// Auth responses
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
}

// User responses
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: String,
}

// Ledger responses
#[derive(Debug, Serialize)]
pub struct LedgerResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub ledger_type: String,
    pub currency: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct LedgerListResponse {
    pub items: Vec<LedgerResponse>,
}

// Bill responses
#[derive(Debug, Serialize)]
pub struct BillResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub bill_type: String,
    pub amount: f64,
    pub category: CategoryBriefResponse,
    pub note: Option<String>,
    pub bill_date: String,
    pub user: UserBriefResponse,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct BillListResponse {
    pub items: Vec<BillResponse>,
    pub pagination: PaginationResponse,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse {
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
    pub total_pages: u32,
}

#[derive(Debug, Serialize)]
pub struct BillStatisticsResponse {
    pub total_income: f64,
    pub total_expense: f64,
    pub balance: f64,
    pub by_category: Vec<CategoryStatisticsResponse>,
}

#[derive(Debug, Serialize)]
pub struct CategoryStatisticsResponse {
    pub category_id: String,
    pub category_name: String,
    pub category_icon: Option<String>,
    #[serde(rename = "type")]
    pub bill_type: String,
    pub amount: f64,
    pub count: u32,
    pub percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct DailyStatisticsResponse {
    pub date: String,
    pub income: f64,
    pub expense: f64,
}

#[derive(Debug, Serialize)]
pub struct TrendStatisticsResponse {
    pub period: String,
    pub income: f64,
    pub expense: f64,
    pub balance: f64,
}

#[derive(Debug, Serialize)]
pub struct FullStatisticsResponse {
    pub summary: BillStatisticsResponse,
    pub daily: Vec<DailyStatisticsResponse>,
    pub trend: Vec<TrendStatisticsResponse>,
}

// Category responses
#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    #[serde(rename = "type")]
    pub category_type: String,
    pub children: Vec<CategoryResponse>,
}

#[derive(Debug, Serialize)]
pub struct CategoryBriefResponse {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CategoryListResponse {
    pub items: Vec<CategoryResponse>,
}

// Group responses
#[derive(Debug, Serialize)]
pub struct GroupResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub member_count: usize,
    pub my_role: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct GroupDetailResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner: UserBriefResponse,
    pub members: Vec<GroupMemberResponse>,
    pub ledgers: Vec<LedgerBriefResponse>,
    pub invite_code: Option<String>,  // Only visible to owner/admin
}

#[derive(Debug, Serialize)]
pub struct GroupMemberResponse {
    pub user_id: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
    pub joined_at: String,
}

#[derive(Debug, Serialize)]
pub struct GroupListResponse {
    pub items: Vec<GroupResponse>,
}

#[derive(Debug, Serialize)]
pub struct CreateGroupResponse {
    pub id: String,
    pub name: String,
    pub invite_code: String,
}

// Brief responses (for embedding)
#[derive(Debug, Serialize)]
pub struct UserBriefResponse {
    pub id: String,
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LedgerBriefResponse {
    pub id: String,
    pub name: String,
}
