use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::dto::{
    ApiResponse, CategoryListResponse, CategoryResponse, CreateCategoryRequest,
};
use crate::error::{AppError, AppResult};
use crate::middleware::CurrentUser;
use crate::models::{BillType, Category, CreateCategory};
use crate::repositories::CategoryRepository;

pub struct CategoryApi {
    category_repo: Arc<dyn CategoryRepository>,
}

impl CategoryApi {
    pub fn new(category_repo: Arc<dyn CategoryRepository>) -> Self {
        Self { category_repo }
    }
}

fn category_to_response(category: &Category, children: Vec<CategoryResponse>) -> CategoryResponse {
    CategoryResponse {
        id: category.id.to_string(),
        name: category.name.clone(),
        icon: category.icon.clone(),
        category_type: category.category_type.to_string(),
        children,
    }
}

pub async fn list_categories(
    State(api): State<Arc<CategoryApi>>,
    Query(params): Query<ListCategoriesParams>,
) -> AppResult<Json<ApiResponse<CategoryListResponse>>> {
    let ledger_id = params.ledger_id.as_ref().and_then(|id| Uuid::parse_str(id).ok());
    let category_type = params.category_type.as_ref().and_then(|t| match t.as_str() {
        "income" => Some(BillType::Income),
        "expense" => Some(BillType::Expense),
        _ => None,
    });

    let categories = api.category_repo.find_by_ledger_id(ledger_id, category_type).await?;

    // Build tree structure
    let mut root_categories: Vec<CategoryResponse> = Vec::new();
    let mut children_map: std::collections::HashMap<Uuid, Vec<Category>> = std::collections::HashMap::new();

    // Group children by parent_id
    for cat in &categories {
        if let Some(parent_id) = cat.parent_id {
            children_map.entry(parent_id).or_default().push(cat.clone());
        }
    }

    // Build response
    for cat in &categories {
        if cat.parent_id.is_none() {
            let children: Vec<CategoryResponse> = children_map
                .get(&cat.id)
                .map(|c| c.iter().map(|child| category_to_response(child, vec![])).collect())
                .unwrap_or_default();

            root_categories.push(category_to_response(cat, children));
        }
    }

    Ok(Json(ApiResponse::success(CategoryListResponse {
        items: root_categories,
    })))
}

pub async fn create_category(
    State(api): State<Arc<CategoryApi>>,
    Extension(_current_user): Extension<CurrentUser>,
    Json(req): Json<CreateCategoryRequest>,
) -> AppResult<Json<ApiResponse<CategoryResponse>>> {
    let category_type = match req.category_type.as_str() {
        "income" => BillType::Income,
        "expense" => BillType::Expense,
        _ => return Err(AppError::Validation("Invalid category type".to_string())),
    };

    let parent_id = req.parent_id.as_ref().and_then(|id| Uuid::parse_str(id).ok());

    let category = api
        .category_repo
        .create(CreateCategory {
            name: req.name,
            icon: req.icon,
            category_type,
            parent_id,
            ledger_id: None, // Custom categories linked to ledger
            sort_order: None,
        })
        .await?;

    Ok(Json(ApiResponse::success(category_to_response(&category, vec![]))))
}

pub async fn update_category(
    State(api): State<Arc<CategoryApi>>,
    Extension(_current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateCategoryRequest>,
) -> AppResult<Json<ApiResponse<CategoryResponse>>> {
    let category_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    let category = api
        .category_repo
        .update(category_id, req.name, req.icon, req.sort_order)
        .await?;

    Ok(Json(ApiResponse::success(category_to_response(&category, vec![]))))
}

pub async fn delete_category(
    State(api): State<Arc<CategoryApi>>,
    Extension(_current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let category_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid category ID".to_string()))?;

    api.category_repo.delete(category_id).await?;

    Ok(Json(ApiResponse::success(())))
}

#[derive(Debug, serde::Deserialize)]
pub struct ListCategoriesParams {
    pub ledger_id: Option<String>,
    #[serde(rename = "type")]
    pub category_type: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}
