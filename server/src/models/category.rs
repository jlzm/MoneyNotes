use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::bill::BillType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub icon: Option<String>,
    pub category_type: BillType,  // income or expense
    pub parent_id: Option<Uuid>,
    pub ledger_id: Option<Uuid>,  // None = system default
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub icon: Option<String>,
    pub category_type: BillType,
    pub parent_id: Option<Uuid>,
    pub ledger_id: Option<Uuid>,
    pub sort_order: Option<i32>,
}

impl Category {
    pub fn new(
        name: String,
        icon: Option<String>,
        category_type: BillType,
        parent_id: Option<Uuid>,
        ledger_id: Option<Uuid>,
        sort_order: Option<i32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            icon,
            category_type,
            parent_id,
            ledger_id,
            sort_order: sort_order.unwrap_or(0),
            created_at: Utc::now(),
        }
    }

    /// Create default expense categories
    pub fn default_expense_categories() -> Vec<Self> {
        vec![
            Self::new("餐饮".to_string(), Some("food".to_string()), BillType::Expense, None, None, Some(1)),
            Self::new("交通".to_string(), Some("transport".to_string()), BillType::Expense, None, None, Some(2)),
            Self::new("购物".to_string(), Some("shopping".to_string()), BillType::Expense, None, None, Some(3)),
            Self::new("娱乐".to_string(), Some("entertainment".to_string()), BillType::Expense, None, None, Some(4)),
            Self::new("居住".to_string(), Some("housing".to_string()), BillType::Expense, None, None, Some(5)),
            Self::new("医疗".to_string(), Some("medical".to_string()), BillType::Expense, None, None, Some(6)),
            Self::new("教育".to_string(), Some("education".to_string()), BillType::Expense, None, None, Some(7)),
            Self::new("通讯".to_string(), Some("communication".to_string()), BillType::Expense, None, None, Some(8)),
            Self::new("其他".to_string(), Some("other".to_string()), BillType::Expense, None, None, Some(99)),
        ]
    }

    /// Create default income categories
    pub fn default_income_categories() -> Vec<Self> {
        vec![
            Self::new("工资".to_string(), Some("salary".to_string()), BillType::Income, None, None, Some(1)),
            Self::new("奖金".to_string(), Some("bonus".to_string()), BillType::Income, None, None, Some(2)),
            Self::new("投资".to_string(), Some("investment".to_string()), BillType::Income, None, None, Some(3)),
            Self::new("兼职".to_string(), Some("part-time".to_string()), BillType::Income, None, None, Some(4)),
            Self::new("红包".to_string(), Some("red-packet".to_string()), BillType::Income, None, None, Some(5)),
            Self::new("其他".to_string(), Some("other".to_string()), BillType::Income, None, None, Some(99)),
        ]
    }
}
