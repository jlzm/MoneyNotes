use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateLedger, Ledger, LedgerType};
use crate::repositories::traits::LedgerRepository;

pub struct MemoryLedgerRepository {
    ledgers: RwLock<HashMap<Uuid, Ledger>>,
}

impl MemoryLedgerRepository {
    pub fn new() -> Self {
        Self {
            ledgers: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryLedgerRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LedgerRepository for MemoryLedgerRepository {
    async fn create(&self, ledger: CreateLedger) -> AppResult<Ledger> {
        let new_ledger = if ledger.ledger_type == LedgerType::Personal {
            Ledger::new_personal(ledger.name, ledger.user_id.unwrap(), ledger.currency)
        } else {
            Ledger::new_group(ledger.name, ledger.group_id.unwrap(), ledger.currency)
        };

        let mut ledgers = self.ledgers.write().unwrap();
        ledgers.insert(new_ledger.id, new_ledger.clone());
        Ok(new_ledger)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Ledger>> {
        let ledgers = self.ledgers.read().unwrap();
        Ok(ledgers.get(&id).cloned())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Ledger>> {
        let ledgers = self.ledgers.read().unwrap();
        Ok(ledgers
            .values()
            .filter(|l| l.user_id == Some(user_id))
            .cloned()
            .collect())
    }

    async fn find_by_group_id(&self, group_id: Uuid) -> AppResult<Vec<Ledger>> {
        let ledgers = self.ledgers.read().unwrap();
        Ok(ledgers
            .values()
            .filter(|l| l.group_id == Some(group_id))
            .cloned()
            .collect())
    }

    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>) -> AppResult<Ledger> {
        let mut ledgers = self.ledgers.write().unwrap();
        let ledger = ledgers.get_mut(&id).ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

        if let Some(n) = name {
            ledger.name = n;
        }
        if let Some(d) = description {
            ledger.description = Some(d);
        }
        ledger.updated_at = chrono::Utc::now();

        Ok(ledger.clone())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut ledgers = self.ledgers.write().unwrap();
        ledgers.remove(&id);
        Ok(())
    }
}
