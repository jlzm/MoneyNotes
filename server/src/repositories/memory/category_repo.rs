use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{BillType, Category, CreateCategory};
use crate::repositories::traits::CategoryRepository;

pub struct MemoryCategoryRepository {
    categories: RwLock<HashMap<Uuid, Category>>,
}

impl MemoryCategoryRepository {
    pub fn new() -> Self {
        Self {
            categories: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryCategoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CategoryRepository for MemoryCategoryRepository {
    async fn create(&self, category: CreateCategory) -> AppResult<Category> {
        let new_category = Category::new(
            category.name,
            category.icon,
            category.category_type,
            category.parent_id,
            category.ledger_id,
            category.sort_order,
        );
        let mut categories = self.categories.write().unwrap();
        categories.insert(new_category.id, new_category.clone());
        Ok(new_category)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Category>> {
        let categories = self.categories.read().unwrap();
        Ok(categories.get(&id).cloned())
    }

    async fn find_by_ledger_id(&self, ledger_id: Option<Uuid>, category_type: Option<BillType>) -> AppResult<Vec<Category>> {
        let categories = self.categories.read().unwrap();
        let mut result: Vec<Category> = categories
            .values()
            .filter(|c| {
                // Show system categories (ledger_id is None) or custom categories for specific ledger
                let ledger_match = c.ledger_id.is_none() || c.ledger_id == ledger_id;
                let type_match = category_type.as_ref().map(|t| &c.category_type == t).unwrap_or(true);
                ledger_match && type_match
            })
            .cloned()
            .collect();
        result.sort_by(|a, b| a.sort_order.cmp(&b.sort_order));
        Ok(result)
    }

    async fn update(&self, id: Uuid, name: Option<String>, icon: Option<String>, sort_order: Option<i32>) -> AppResult<Category> {
        let mut categories = self.categories.write().unwrap();
        let category = categories.get_mut(&id).ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

        if let Some(n) = name {
            category.name = n;
        }
        if let Some(i) = icon {
            category.icon = Some(i);
        }
        if let Some(s) = sort_order {
            category.sort_order = s;
        }

        Ok(category.clone())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut categories = self.categories.write().unwrap();
        categories.remove(&id);
        Ok(())
    }

    async fn init_default_categories(&self) -> AppResult<()> {
        // Check if categories already exist
        let is_empty = {
            let categories = self.categories.read().unwrap();
            categories.is_empty()
        };

        if !is_empty {
            return Ok(());
        }

        // Insert expense categories
        for cat in Category::default_expense_categories() {
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
        for cat in Category::default_income_categories() {
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
