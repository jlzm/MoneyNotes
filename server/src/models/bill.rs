use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BillType {
    Income,
    Expense,
}

impl std::fmt::Display for BillType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BillType::Income => write!(f, "income"),
            BillType::Expense => write!(f, "expense"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bill {
    pub id: Uuid,
    pub ledger_id: Uuid,
    pub category_id: Uuid,
    pub user_id: Uuid,  // Who created this bill
    pub bill_type: BillType,
    pub amount: f64,
    pub note: Option<String>,
    pub bill_date: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBill {
    pub ledger_id: Uuid,
    pub category_id: Uuid,
    pub user_id: Uuid,
    pub bill_type: BillType,
    pub amount: f64,
    pub note: Option<String>,
    pub bill_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBill {
    pub category_id: Option<Uuid>,
    pub bill_type: Option<BillType>,
    pub amount: Option<f64>,
    pub note: Option<String>,
    pub bill_date: Option<NaiveDate>,
}

impl Bill {
    pub fn new(
        ledger_id: Uuid,
        category_id: Uuid,
        user_id: Uuid,
        bill_type: BillType,
        amount: f64,
        note: Option<String>,
        bill_date: NaiveDate,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            ledger_id,
            category_id,
            user_id,
            bill_type,
            amount,
            note,
            bill_date,
            created_at: now,
            updated_at: now,
        }
    }
}
