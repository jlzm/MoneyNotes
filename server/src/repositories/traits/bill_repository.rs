use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::error::AppResult;
use crate::models::{Bill, BillType, CreateBill, UpdateBill};

#[derive(Debug, Clone)]
pub struct BillFilter {
    pub ledger_id: Uuid,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub bill_type: Option<BillType>,
    pub category_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone)]
pub struct BillStatistics {
    pub total_income: f64,
    pub total_expense: f64,
    pub balance: f64,
}

#[derive(Debug, Clone)]
pub struct CategoryStatistics {
    pub category_id: Uuid,
    pub category_name: String,
    pub category_icon: Option<String>,
    pub bill_type: BillType,
    pub amount: f64,
    pub count: u32,
    pub percentage: f64,
}

#[derive(Debug, Clone)]
pub struct DailyStatistics {
    pub date: NaiveDate,
    pub income: f64,
    pub expense: f64,
}

#[derive(Debug, Clone)]
pub struct TrendStatistics {
    pub period: String,  // e.g., "2025-01", "2025-W01", "2025-01-01"
    pub income: f64,
    pub expense: f64,
    pub balance: f64,
}

#[async_trait]
pub trait BillRepository: Send + Sync {
    async fn create(&self, bill: CreateBill) -> AppResult<Bill>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Bill>>;
    async fn find_by_filter(&self, filter: BillFilter) -> AppResult<(Vec<Bill>, u64)>;
    async fn update(&self, id: Uuid, bill: UpdateBill) -> AppResult<Bill>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;

    // 基础统计
    async fn get_statistics(&self, ledger_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> AppResult<BillStatistics>;

    // 分类统计
    async fn get_category_statistics(&self, ledger_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, bill_type: Option<BillType>) -> AppResult<Vec<CategoryStatistics>>;

    // 每日统计
    async fn get_daily_statistics(&self, ledger_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> AppResult<Vec<DailyStatistics>>;

    // 趋势统计 (按月/周/日)
    async fn get_trend_statistics(&self, ledger_id: Uuid, start_date: NaiveDate, end_date: NaiveDate, group_by: &str) -> AppResult<Vec<TrendStatistics>>;
}
