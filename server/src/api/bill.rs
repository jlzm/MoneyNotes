use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::dto::{
    ApiResponse, BillListResponse, BillQueryParams, BillResponse, BillStatisticsResponse,
    CategoryBriefResponse, CategoryStatisticsResponse, CreateBillRequest, DailyStatisticsResponse,
    FullStatisticsResponse, PaginationResponse, TrendStatisticsResponse, UpdateBillRequest,
    UserBriefResponse,
};
use crate::error::{AppError, AppResult};
use crate::middleware::CurrentUser;
use crate::models::{BillType, CreateBill, UpdateBill};
use crate::repositories::{BillFilter, BillRepository, CategoryRepository, LedgerRepository, UserRepository};

pub struct BillApi {
    bill_repo: Arc<dyn BillRepository>,
    ledger_repo: Arc<dyn LedgerRepository>,
    category_repo: Arc<dyn CategoryRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl BillApi {
    pub fn new(
        bill_repo: Arc<dyn BillRepository>,
        ledger_repo: Arc<dyn LedgerRepository>,
        category_repo: Arc<dyn CategoryRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            bill_repo,
            ledger_repo,
            category_repo,
            user_repo,
        }
    }
}

pub async fn list_bills(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(params): Query<BillQueryParams>,
) -> AppResult<Json<ApiResponse<BillListResponse>>> {
    let ledger_id = Uuid::parse_str(&params.ledger_id)
        .map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    // Check access to ledger
    let ledger = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if ledger.user_id != Some(current_user.id) && ledger.group_id.is_none() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let bill_type = params.bill_type.as_ref().and_then(|t| match t.as_str() {
        "income" => Some(BillType::Income),
        "expense" => Some(BillType::Expense),
        _ => None,
    });

    let filter = BillFilter {
        ledger_id,
        start_date: params.start_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
        end_date: params.end_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()),
        bill_type,
        category_id: params.category_id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
        user_id: None,
        page: params.page.unwrap_or(1),
        page_size: params.page_size.unwrap_or(20),
    };

    let (bills, total) = api.bill_repo.find_by_filter(filter.clone()).await?;

    let mut items = Vec::new();
    for bill in bills {
        let category = api.category_repo.find_by_id(bill.category_id).await?.unwrap();
        let user = api.user_repo.find_by_id(bill.user_id).await?.unwrap();

        items.push(BillResponse {
            id: bill.id.to_string(),
            bill_type: bill.bill_type.to_string(),
            amount: bill.amount,
            category: CategoryBriefResponse {
                id: category.id.to_string(),
                name: category.name,
                icon: category.icon,
            },
            note: bill.note,
            bill_date: bill.bill_date.to_string(),
            user: UserBriefResponse {
                id: user.id.to_string(),
                nickname: user.nickname,
            },
            created_at: bill.created_at.to_rfc3339(),
        });
    }

    let total_pages = ((total as f64) / (filter.page_size as f64)).ceil() as u32;

    Ok(Json(ApiResponse::success(BillListResponse {
        items,
        pagination: PaginationResponse {
            page: filter.page,
            page_size: filter.page_size,
            total,
            total_pages,
        },
    })))
}

pub async fn create_bill(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<CreateBillRequest>,
) -> AppResult<Json<ApiResponse<BillResponse>>> {
    let ledger_id = Uuid::parse_str(&req.ledger_id)
        .map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;
    let category_id = Uuid::parse_str(&req.category_id)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    // Check access to ledger
    let ledger = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if ledger.user_id != Some(current_user.id) && ledger.group_id.is_none() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let bill_type = match req.bill_type.as_str() {
        "income" => BillType::Income,
        "expense" => BillType::Expense,
        _ => return Err(AppError::Validation("Invalid bill type".to_string())),
    };

    let bill_date = NaiveDate::parse_from_str(&req.bill_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid date format".to_string()))?;

    let bill = api
        .bill_repo
        .create(CreateBill {
            ledger_id,
            category_id,
            user_id: current_user.id,
            bill_type,
            amount: req.amount,
            note: req.note,
            bill_date,
        })
        .await?;

    let category = api.category_repo.find_by_id(bill.category_id).await?.unwrap();
    let user = api.user_repo.find_by_id(bill.user_id).await?.unwrap();

    Ok(Json(ApiResponse::success(BillResponse {
        id: bill.id.to_string(),
        bill_type: bill.bill_type.to_string(),
        amount: bill.amount,
        category: CategoryBriefResponse {
            id: category.id.to_string(),
            name: category.name,
            icon: category.icon,
        },
        note: bill.note,
        bill_date: bill.bill_date.to_string(),
        user: UserBriefResponse {
            id: user.id.to_string(),
            nickname: user.nickname,
        },
        created_at: bill.created_at.to_rfc3339(),
    })))
}

pub async fn get_bill(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<BillResponse>>> {
    let bill_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid bill ID".to_string()))?;

    let bill = api
        .bill_repo
        .find_by_id(bill_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Bill not found".to_string()))?;

    // Check access
    let ledger = api.ledger_repo.find_by_id(bill.ledger_id).await?.unwrap();
    if ledger.user_id != Some(current_user.id) && ledger.group_id.is_none() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let category = api.category_repo.find_by_id(bill.category_id).await?.unwrap();
    let user = api.user_repo.find_by_id(bill.user_id).await?.unwrap();

    Ok(Json(ApiResponse::success(BillResponse {
        id: bill.id.to_string(),
        bill_type: bill.bill_type.to_string(),
        amount: bill.amount,
        category: CategoryBriefResponse {
            id: category.id.to_string(),
            name: category.name,
            icon: category.icon,
        },
        note: bill.note,
        bill_date: bill.bill_date.to_string(),
        user: UserBriefResponse {
            id: user.id.to_string(),
            nickname: user.nickname,
        },
        created_at: bill.created_at.to_rfc3339(),
    })))
}

pub async fn update_bill(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateBillRequest>,
) -> AppResult<Json<ApiResponse<BillResponse>>> {
    let bill_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid bill ID".to_string()))?;

    let existing = api
        .bill_repo
        .find_by_id(bill_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Bill not found".to_string()))?;

    // Only creator can update
    if existing.user_id != current_user.id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let bill_type = req.bill_type.as_ref().and_then(|t| match t.as_str() {
        "income" => Some(BillType::Income),
        "expense" => Some(BillType::Expense),
        _ => None,
    });

    let bill_date = req.bill_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());

    let bill = api
        .bill_repo
        .update(
            bill_id,
            UpdateBill {
                category_id: req.category_id.as_ref().and_then(|id| Uuid::parse_str(id).ok()),
                bill_type,
                amount: req.amount,
                note: req.note.clone(),
                bill_date,
            },
        )
        .await?;

    let category = api.category_repo.find_by_id(bill.category_id).await?.unwrap();
    let user = api.user_repo.find_by_id(bill.user_id).await?.unwrap();

    Ok(Json(ApiResponse::success(BillResponse {
        id: bill.id.to_string(),
        bill_type: bill.bill_type.to_string(),
        amount: bill.amount,
        category: CategoryBriefResponse {
            id: category.id.to_string(),
            name: category.name,
            icon: category.icon,
        },
        note: bill.note,
        bill_date: bill.bill_date.to_string(),
        user: UserBriefResponse {
            id: user.id.to_string(),
            nickname: user.nickname,
        },
        created_at: bill.created_at.to_rfc3339(),
    })))
}

pub async fn delete_bill(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let bill_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid bill ID".to_string()))?;

    let existing = api
        .bill_repo
        .find_by_id(bill_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Bill not found".to_string()))?;

    // Only creator can delete
    if existing.user_id != current_user.id {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    api.bill_repo.delete(bill_id).await?;

    Ok(Json(ApiResponse::success(())))
}

#[derive(Debug, serde::Deserialize)]
pub struct StatisticsParams {
    pub ledger_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub period: Option<String>,  // day, week, month, year
    pub bill_type: Option<String>,  // income, expense
}

pub async fn get_statistics(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(params): Query<StatisticsParams>,
) -> AppResult<Json<ApiResponse<FullStatisticsResponse>>> {
    let ledger_id = Uuid::parse_str(&params.ledger_id)
        .map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    // Check access
    let ledger = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if ledger.user_id != Some(current_user.id) && ledger.group_id.is_none() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let period = params.period.as_deref().unwrap_or("month");

    // Calculate date range based on period
    let today = Utc::now().date_naive();
    let (start_date, end_date) = if let (Some(s), Some(e)) = (&params.start_date, &params.end_date) {
        (
            NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap_or(today - Duration::days(30)),
            NaiveDate::parse_from_str(e, "%Y-%m-%d").unwrap_or(today),
        )
    } else {
        match period {
            "day" => (today, today),
            "week" => {
                let start = today - Duration::days(today.weekday().num_days_from_monday() as i64);
                (start, start + Duration::days(6))
            }
            "month" => {
                let start = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
                let next_month = if today.month() == 12 {
                    NaiveDate::from_ymd_opt(today.year() + 1, 1, 1).unwrap()
                } else {
                    NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1).unwrap()
                };
                (start, next_month - Duration::days(1))
            }
            "year" => {
                let start = NaiveDate::from_ymd_opt(today.year(), 1, 1).unwrap();
                let end = NaiveDate::from_ymd_opt(today.year(), 12, 31).unwrap();
                (start, end)
            }
            _ => (today - Duration::days(30), today),
        }
    };

    let bill_type = params.bill_type.as_ref().and_then(|t| match t.as_str() {
        "income" => Some(BillType::Income),
        "expense" => Some(BillType::Expense),
        _ => None,
    });

    // Get basic statistics
    let stats = api.bill_repo.get_statistics(ledger_id, Some(start_date), Some(end_date)).await?;

    // Get category statistics
    let category_stats = api.bill_repo.get_category_statistics(ledger_id, Some(start_date), Some(end_date), bill_type).await?;

    let mut by_category: Vec<CategoryStatisticsResponse> = Vec::new();
    for c in category_stats {
        let category = api.category_repo.find_by_id(c.category_id).await?;
        by_category.push(CategoryStatisticsResponse {
            category_id: c.category_id.to_string(),
            category_name: category.as_ref().map(|cat| cat.name.clone()).unwrap_or_else(|| c.category_name),
            category_icon: category.and_then(|cat| cat.icon),
            bill_type: c.bill_type.to_string(),
            amount: c.amount,
            count: c.count,
            percentage: c.percentage,
        });
    }

    // Get daily statistics
    let daily_stats = api.bill_repo.get_daily_statistics(ledger_id, start_date, end_date).await?;
    let daily: Vec<DailyStatisticsResponse> = daily_stats
        .into_iter()
        .map(|d| DailyStatisticsResponse {
            date: d.date.format("%Y-%m-%d").to_string(),
            income: d.income,
            expense: d.expense,
        })
        .collect();

    // Get trend statistics
    let group_by = match period {
        "day" | "week" => "day",
        "month" => "day",
        "year" => "month",
        _ => "day",
    };
    let trend_stats = api.bill_repo.get_trend_statistics(ledger_id, start_date, end_date, group_by).await?;
    let trend: Vec<TrendStatisticsResponse> = trend_stats
        .into_iter()
        .map(|t| TrendStatisticsResponse {
            period: t.period,
            income: t.income,
            expense: t.expense,
            balance: t.balance,
        })
        .collect();

    Ok(Json(ApiResponse::success(FullStatisticsResponse {
        summary: BillStatisticsResponse {
            total_income: stats.total_income,
            total_expense: stats.total_expense,
            balance: stats.balance,
            by_category,
        },
        daily,
        trend,
    })))
}

#[derive(Debug, serde::Deserialize)]
pub struct CategoryStatsParams {
    pub ledger_id: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub bill_type: Option<String>,
}

pub async fn get_category_statistics(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(params): Query<CategoryStatsParams>,
) -> AppResult<Json<ApiResponse<Vec<CategoryStatisticsResponse>>>> {
    let ledger_id = Uuid::parse_str(&params.ledger_id)
        .map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    // Check access
    let ledger = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if ledger.user_id != Some(current_user.id) && ledger.group_id.is_none() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let start_date = params.start_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let end_date = params.end_date.as_ref().and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok());
    let bill_type = params.bill_type.as_ref().and_then(|t| match t.as_str() {
        "income" => Some(BillType::Income),
        "expense" => Some(BillType::Expense),
        _ => None,
    });

    let category_stats = api.bill_repo.get_category_statistics(ledger_id, start_date, end_date, bill_type).await?;

    let mut result: Vec<CategoryStatisticsResponse> = Vec::new();
    for c in category_stats {
        let category = api.category_repo.find_by_id(c.category_id).await?;
        result.push(CategoryStatisticsResponse {
            category_id: c.category_id.to_string(),
            category_name: category.as_ref().map(|cat| cat.name.clone()).unwrap_or_else(|| c.category_name),
            category_icon: category.and_then(|cat| cat.icon),
            bill_type: c.bill_type.to_string(),
            amount: c.amount,
            count: c.count,
            percentage: c.percentage,
        });
    }

    Ok(Json(ApiResponse::success(result)))
}

#[derive(Debug, serde::Deserialize)]
pub struct TrendParams {
    pub ledger_id: String,
    pub start_date: String,
    pub end_date: String,
    pub group_by: Option<String>,  // day, week, month, year
}

pub async fn get_trend_statistics(
    State(api): State<Arc<BillApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(params): Query<TrendParams>,
) -> AppResult<Json<ApiResponse<Vec<TrendStatisticsResponse>>>> {
    let ledger_id = Uuid::parse_str(&params.ledger_id)
        .map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    // Check access
    let ledger = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if ledger.user_id != Some(current_user.id) && ledger.group_id.is_none() {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let start_date = NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid start date".to_string()))?;
    let end_date = NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("Invalid end date".to_string()))?;
    let group_by = params.group_by.as_deref().unwrap_or("month");

    let trend_stats = api.bill_repo.get_trend_statistics(ledger_id, start_date, end_date, group_by).await?;

    let result: Vec<TrendStatisticsResponse> = trend_stats
        .into_iter()
        .map(|t| TrendStatisticsResponse {
            period: t.period,
            income: t.income,
            expense: t.expense,
            balance: t.balance,
        })
        .collect();

    Ok(Json(ApiResponse::success(result)))
}
