use async_trait::async_trait;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateLedger, Ledger, LedgerType};
use crate::repositories::traits::LedgerRepository;

pub struct MySqlLedgerRepository {
    pool: MySqlPool,
}

impl MySqlLedgerRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LedgerRepository for MySqlLedgerRepository {
    async fn create(&self, ledger: CreateLedger) -> AppResult<Ledger> {
        let new_ledger = if ledger.ledger_type == LedgerType::Personal {
            Ledger::new_personal(ledger.name, ledger.user_id.unwrap(), ledger.currency)
        } else {
            Ledger::new_group(ledger.name, ledger.group_id.unwrap(), ledger.currency)
        };

        sqlx::query(
            r#"
            INSERT INTO ledgers (id, name, description, type, user_id, group_id, currency, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_ledger.id.to_string())
        .bind(&new_ledger.name)
        .bind(&new_ledger.description)
        .bind(new_ledger.ledger_type.to_string())
        .bind(new_ledger.user_id.map(|id| id.to_string()))
        .bind(new_ledger.group_id.map(|id| id.to_string()))
        .bind(&new_ledger.currency)
        .bind(new_ledger.created_at)
        .bind(new_ledger.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(new_ledger)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Ledger>> {
        let result = sqlx::query_as::<_, LedgerRow>(
            r#"
            SELECT id, name, description, type, user_id, group_id, currency, created_at, updated_at
            FROM ledgers WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Ledger>> {
        let rows = sqlx::query_as::<_, LedgerRow>(
            r#"
            SELECT id, name, description, type, user_id, group_id, currency, created_at, updated_at
            FROM ledgers WHERE user_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(user_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn find_by_group_id(&self, group_id: Uuid) -> AppResult<Vec<Ledger>> {
        let rows = sqlx::query_as::<_, LedgerRow>(
            r#"
            SELECT id, name, description, type, user_id, group_id, currency, created_at, updated_at
            FROM ledgers WHERE group_id = ? ORDER BY created_at DESC
            "#,
        )
        .bind(group_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>) -> AppResult<Ledger> {
        sqlx::query(
            r#"
            UPDATE ledgers SET name = COALESCE(?, name), description = COALESCE(?, description), updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(&name)
        .bind(&description)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM ledgers WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct LedgerRow {
    id: String,
    name: String,
    description: Option<String>,
    #[sqlx(rename = "type")]
    ledger_type: String,
    user_id: Option<String>,
    group_id: Option<String>,
    currency: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<LedgerRow> for Ledger {
    fn from(row: LedgerRow) -> Self {
        Ledger {
            id: Uuid::parse_str(&row.id).unwrap(),
            name: row.name,
            description: row.description,
            ledger_type: match row.ledger_type.as_str() {
                "group" => LedgerType::Group,
                _ => LedgerType::Personal,
            },
            user_id: row.user_id.and_then(|id| Uuid::parse_str(&id).ok()),
            group_id: row.group_id.and_then(|id| Uuid::parse_str(&id).ok()),
            currency: row.currency,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
