use async_trait::async_trait;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{BillType, Category, CreateCategory};
use crate::repositories::traits::CategoryRepository;

pub struct MySqlCategoryRepository {
    pool: MySqlPool,
}

impl MySqlCategoryRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for MySqlCategoryRepository {
    async fn create(&self, category: CreateCategory) -> AppResult<Category> {
        let new_category = Category::new(
            category.name,
            category.icon,
            category.category_type,
            category.parent_id,
            category.ledger_id,
            category.sort_order,
        );

        sqlx::query(
            r#"
            INSERT INTO categories (id, name, icon, type, parent_id, ledger_id, sort_order, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_category.id.to_string())
        .bind(&new_category.name)
        .bind(&new_category.icon)
        .bind(new_category.category_type.to_string())
        .bind(new_category.parent_id.map(|id| id.to_string()))
        .bind(new_category.ledger_id.map(|id| id.to_string()))
        .bind(new_category.sort_order)
        .bind(new_category.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(new_category)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Category>> {
        let result = sqlx::query_as::<_, CategoryRow>(
            r#"
            SELECT id, name, icon, type, parent_id, ledger_id, sort_order, created_at
            FROM categories WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }

    async fn find_by_ledger_id(&self, ledger_id: Option<Uuid>, category_type: Option<BillType>) -> AppResult<Vec<Category>> {
        let mut query = String::from(
            "SELECT id, name, icon, type, parent_id, ledger_id, sort_order, created_at FROM categories WHERE 1=1"
        );

        if ledger_id.is_some() {
            query.push_str(" AND (ledger_id = ? OR ledger_id IS NULL)");
        } else {
            query.push_str(" AND ledger_id IS NULL");
        }

        if category_type.is_some() {
            query.push_str(" AND type = ?");
        }

        query.push_str(" ORDER BY sort_order ASC, name ASC");

        let mut q = sqlx::query_as::<_, CategoryRow>(&query);

        if let Some(id) = ledger_id {
            q = q.bind(id.to_string());
        }

        if let Some(ref t) = category_type {
            q = q.bind(t.to_string());
        }

        let rows = q
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, id: Uuid, name: Option<String>, icon: Option<String>, sort_order: Option<i32>) -> AppResult<Category> {
        sqlx::query(
            r#"
            UPDATE categories SET name = COALESCE(?, name), icon = COALESCE(?, icon), sort_order = COALESCE(?, sort_order)
            WHERE id = ?
            "#,
        )
        .bind(&name)
        .bind(&icon)
        .bind(sort_order)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Category not found".to_string()))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM categories WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn init_default_categories(&self) -> AppResult<()> {
        // Check if default categories exist
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE ledger_id IS NULL")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        if count > 0 {
            return Ok(());
        }

        // Insert expense categories
        let expense_categories = Category::default_expense_categories();
        for cat in expense_categories {
            self.create(CreateCategory {
                name: cat.name,
                icon: cat.icon,
                category_type: cat.category_type,
                parent_id: None,
                ledger_id: None,
                sort_order: Some(cat.sort_order),
            })
            .await?;
        }

        // Insert income categories
        let income_categories = Category::default_income_categories();
        for cat in income_categories {
            self.create(CreateCategory {
                name: cat.name,
                icon: cat.icon,
                category_type: cat.category_type,
                parent_id: None,
                ledger_id: None,
                sort_order: Some(cat.sort_order),
            })
            .await?;
        }

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct CategoryRow {
    id: String,
    name: String,
    icon: Option<String>,
    #[sqlx(rename = "type")]
    category_type: String,
    parent_id: Option<String>,
    ledger_id: Option<String>,
    sort_order: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<CategoryRow> for Category {
    fn from(row: CategoryRow) -> Self {
        Category {
            id: Uuid::parse_str(&row.id).unwrap(),
            name: row.name,
            icon: row.icon,
            category_type: match row.category_type.as_str() {
                "income" => BillType::Income,
                _ => BillType::Expense,
            },
            parent_id: row.parent_id.and_then(|id| Uuid::parse_str(&id).ok()),
            ledger_id: row.ledger_id.and_then(|id| Uuid::parse_str(&id).ok()),
            sort_order: row.sort_order,
            created_at: row.created_at,
        }
    }
}
