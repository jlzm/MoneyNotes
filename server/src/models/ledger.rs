use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LedgerType {
    Personal,
    Group,
}

impl std::fmt::Display for LedgerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LedgerType::Personal => write!(f, "personal"),
            LedgerType::Group => write!(f, "group"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ledger {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub ledger_type: LedgerType,
    pub user_id: Option<Uuid>,   // For personal ledger
    pub group_id: Option<Uuid>,  // For group ledger
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLedger {
    pub name: String,
    pub description: Option<String>,
    pub ledger_type: LedgerType,
    pub user_id: Option<Uuid>,
    pub group_id: Option<Uuid>,
    pub currency: Option<String>,
}

impl Ledger {
    pub fn new_personal(name: String, user_id: Uuid, currency: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            ledger_type: LedgerType::Personal,
            user_id: Some(user_id),
            group_id: None,
            currency: currency.unwrap_or_else(|| "CNY".to_string()),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_group(name: String, group_id: Uuid, currency: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description: None,
            ledger_type: LedgerType::Group,
            user_id: None,
            group_id: Some(group_id),
            currency: currency.unwrap_or_else(|| "CNY".to_string()),
            created_at: now,
            updated_at: now,
        }
    }
}
