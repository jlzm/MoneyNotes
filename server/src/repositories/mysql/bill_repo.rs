use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{Bill, BillType, CreateBill, UpdateBill};
use crate::repositories::traits::{BillFilter, BillRepository, BillStatistics, CategoryStatistics, DailyStatistics, TrendStatistics};

pub struct MySqlBillRepository {
    pool: MySqlPool,
}

impl MySqlBillRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BillRepository for MySqlBillRepository {
    async fn create(&self, bill: CreateBill) -> AppResult<Bill> {
        let new_bill = Bill::new(
            bill.ledger_id,
            bill.category_id,
            bill.user_id,
            bill.bill_type,
            bill.amount,
            bill.note,
            bill.bill_date,
        );

        sqlx::query(
            r#"
            INSERT INTO bills (id, ledger_id, category_id, user_id, type, amount, note, bill_date, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_bill.id.to_string())
        .bind(new_bill.ledger_id.to_string())
        .bind(new_bill.category_id.to_string())
        .bind(new_bill.user_id.to_string())
        .bind(new_bill.bill_type.to_string())
        .bind(new_bill.amount)
        .bind(&new_bill.note)
        .bind(new_bill.bill_date)
        .bind(new_bill.created_at)
        .bind(new_bill.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(new_bill)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Bill>> {
        let result = sqlx::query_as::<_, BillRow>(
            r#"
            SELECT id, ledger_id, category_id, user_id, type, amount, note, bill_date, created_at, updated_at
            FROM bills WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }

    async fn find_by_filter(&self, filter: BillFilter) -> AppResult<(Vec<Bill>, u64)> {
        let mut conditions = vec!["ledger_id = ?".to_string()];
        let mut count_conditions = vec!["ledger_id = ?".to_string()];

        if filter.start_date.is_some() {
            conditions.push("bill_date >= ?".to_string());
            count_conditions.push("bill_date >= ?".to_string());
        }
        if filter.end_date.is_some() {
            conditions.push("bill_date <= ?".to_string());
            count_conditions.push("bill_date <= ?".to_string());
        }
        if filter.bill_type.is_some() {
            conditions.push("type = ?".to_string());
            count_conditions.push("type = ?".to_string());
        }
        if filter.category_id.is_some() {
            conditions.push("category_id = ?".to_string());
            count_conditions.push("category_id = ?".to_string());
        }

        let where_clause = conditions.join(" AND ");
        let offset = (filter.page - 1) * filter.page_size;

        let query = format!(
            r#"
            SELECT id, ledger_id, category_id, user_id, type, amount, note, bill_date, created_at, updated_at
            FROM bills WHERE {} ORDER BY bill_date DESC, created_at DESC LIMIT ? OFFSET ?
            "#,
            where_clause
        );

        let count_query = format!("SELECT COUNT(*) as count FROM bills WHERE {}", count_conditions.join(" AND "));

        let mut q = sqlx::query_as::<_, BillRow>(&query).bind(filter.ledger_id.to_string());

        if let Some(date) = filter.start_date {
            q = q.bind(date);
        }
        if let Some(date) = filter.end_date {
            q = q.bind(date);
        }
        if let Some(ref t) = filter.bill_type {
            q = q.bind(t.to_string());
        }
        if let Some(id) = filter.category_id {
            q = q.bind(id.to_string());
        }

        let bills: Vec<Bill> = q
            .bind(filter.page_size as i64)
            .bind(offset as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .into_iter()
            .map(|r| r.into())
            .collect();

        // Get total count
        let mut cq = sqlx::query_scalar::<_, i64>(&count_query).bind(filter.ledger_id.to_string());
        if let Some(date) = filter.start_date {
            cq = cq.bind(date);
        }
        if let Some(date) = filter.end_date {
            cq = cq.bind(date);
        }
        if let Some(ref t) = filter.bill_type {
            cq = cq.bind(t.to_string());
        }
        if let Some(id) = filter.category_id {
            cq = cq.bind(id.to_string());
        }

        let total = cq
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))? as u64;

        Ok((bills, total))
    }

    async fn update(&self, id: Uuid, bill: UpdateBill) -> AppResult<Bill> {
        let existing = self.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Bill not found".to_string()))?;

        let category_id = bill.category_id.unwrap_or(existing.category_id);
        let bill_type = bill.bill_type.unwrap_or(existing.bill_type);
        let amount = bill.amount.unwrap_or(existing.amount);
        let note = bill.note.or(existing.note);
        let bill_date = bill.bill_date.unwrap_or(existing.bill_date);

        sqlx::query(
            r#"
            UPDATE bills SET category_id = ?, type = ?, amount = ?, note = ?, bill_date = ?, updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(category_id.to_string())
        .bind(bill_type.to_string())
        .bind(amount)
        .bind(&note)
        .bind(bill_date)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id).await?.ok_or_else(|| AppError::NotFound("Bill not found".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM bills WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn get_statistics(&self, ledger_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>) -> AppResult<BillStatistics> {
        let mut query = String::from(
            r#"
            SELECT
                COALESCE(SUM(CASE WHEN type = 'income' THEN amount ELSE 0 END), 0) as total_income,
                COALESCE(SUM(CASE WHEN type = 'expense' THEN amount ELSE 0 END), 0) as total_expense
            FROM bills WHERE ledger_id = ?
            "#
        );

        if start_date.is_some() {
            query.push_str(" AND bill_date >= ?");
        }
        if end_date.is_some() {
            query.push_str(" AND bill_date <= ?");
        }

        let mut q = sqlx::query_as::<_, (f64, f64)>(&query).bind(ledger_id.to_string());

        if let Some(date) = start_date {
            q = q.bind(date);
        }
        if let Some(date) = end_date {
            q = q.bind(date);
        }

        let (total_income, total_expense) = q
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(BillStatistics {
            total_income,
            total_expense,
            balance: total_income - total_expense,
        })
    }

    async fn get_category_statistics(&self, ledger_id: Uuid, start_date: Option<NaiveDate>, end_date: Option<NaiveDate>, bill_type: Option<BillType>) -> AppResult<Vec<CategoryStatistics>> {
        let target_type = bill_type.unwrap_or(BillType::Expense);
        let mut query = String::from(
            r#"
            SELECT
                b.category_id,
                c.name as category_name,
                c.icon as category_icon,
                COUNT(*) as count,
                SUM(b.amount) as amount
            FROM bills b
            JOIN categories c ON b.category_id = c.id
            WHERE b.ledger_id = ? AND b.type = ?
            "#
        );

        if start_date.is_some() {
            query.push_str(" AND b.bill_date >= ?");
        }
        if end_date.is_some() {
            query.push_str(" AND b.bill_date <= ?");
        }

        query.push_str(" GROUP BY b.category_id, c.name, c.icon ORDER BY amount DESC");

        let mut q = sqlx::query_as::<_, (String, String, Option<String>, i64, f64)>(&query)
            .bind(ledger_id.to_string())
            .bind(target_type.to_string());

        if let Some(date) = start_date {
            q = q.bind(date);
        }
        if let Some(date) = end_date {
            q = q.bind(date);
        }

        let rows = q
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let total: f64 = rows.iter().map(|(_, _, _, _, a)| a).sum();

        let stats: Vec<CategoryStatistics> = rows
            .into_iter()
            .map(|(id, name, icon, count, amount)| CategoryStatistics {
                category_id: Uuid::parse_str(&id).unwrap(),
                category_name: name,
                category_icon: icon,
                bill_type: target_type.clone(),
                amount,
                count: count as u32,
                percentage: if total > 0.0 { (amount / total) * 100.0 } else { 0.0 },
            })
            .collect();

        Ok(stats)
    }

    async fn get_daily_statistics(&self, ledger_id: Uuid, start_date: NaiveDate, end_date: NaiveDate) -> AppResult<Vec<DailyStatistics>> {
        let query = r#"
            SELECT
                bill_date,
                COALESCE(SUM(CASE WHEN type = 'income' THEN amount ELSE 0 END), 0) as income,
                COALESCE(SUM(CASE WHEN type = 'expense' THEN amount ELSE 0 END), 0) as expense
            FROM bills
            WHERE ledger_id = ? AND bill_date >= ? AND bill_date <= ?
            GROUP BY bill_date
            ORDER BY bill_date ASC
        "#;

        let rows = sqlx::query_as::<_, (NaiveDate, f64, f64)>(query)
            .bind(ledger_id.to_string())
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let stats: Vec<DailyStatistics> = rows
            .into_iter()
            .map(|(date, income, expense)| DailyStatistics { date, income, expense })
            .collect();

        Ok(stats)
    }

    async fn get_trend_statistics(&self, ledger_id: Uuid, start_date: NaiveDate, end_date: NaiveDate, group_by: &str) -> AppResult<Vec<TrendStatistics>> {
        let date_format = match group_by {
            "day" => "%Y-%m-%d",
            "week" => "%x-W%v",  // ISO week format
            "month" => "%Y-%m",
            "year" => "%Y",
            _ => "%Y-%m",
        };

        let query = format!(
            r#"
            SELECT
                DATE_FORMAT(bill_date, '{}') as period,
                COALESCE(SUM(CASE WHEN type = 'income' THEN amount ELSE 0 END), 0) as income,
                COALESCE(SUM(CASE WHEN type = 'expense' THEN amount ELSE 0 END), 0) as expense
            FROM bills
            WHERE ledger_id = ? AND bill_date >= ? AND bill_date <= ?
            GROUP BY period
            ORDER BY period ASC
            "#,
            date_format
        );

        let rows = sqlx::query_as::<_, (String, f64, f64)>(&query)
            .bind(ledger_id.to_string())
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let stats: Vec<TrendStatistics> = rows
            .into_iter()
            .map(|(period, income, expense)| TrendStatistics {
                period,
                income,
                expense,
                balance: income - expense,
            })
            .collect();

        Ok(stats)
    }
}

#[derive(sqlx::FromRow)]
struct BillRow {
    id: String,
    ledger_id: String,
    category_id: String,
    user_id: String,
    #[sqlx(rename = "type")]
    bill_type: String,
    amount: f64,
    note: Option<String>,
    bill_date: chrono::NaiveDate,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<BillRow> for Bill {
    fn from(row: BillRow) -> Self {
        Bill {
            id: Uuid::parse_str(&row.id).unwrap(),
            ledger_id: Uuid::parse_str(&row.ledger_id).unwrap(),
            category_id: Uuid::parse_str(&row.category_id).unwrap(),
            user_id: Uuid::parse_str(&row.user_id).unwrap(),
            bill_type: match row.bill_type.as_str() {
                "income" => BillType::Income,
                _ => BillType::Expense,
            },
            amount: row.amount,
            note: row.note,
            bill_date: row.bill_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}
