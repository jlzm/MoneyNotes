use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::dto::{
    ApiResponse, CreateLedgerRequest, LedgerListResponse, LedgerResponse,
};
use crate::error::{AppError, AppResult};
use crate::middleware::CurrentUser;
use crate::models::{CreateLedger, Ledger, LedgerType};
use crate::repositories::LedgerRepository;

pub struct LedgerApi {
    ledger_repo: Arc<dyn LedgerRepository>,
}

impl LedgerApi {
    pub fn new(ledger_repo: Arc<dyn LedgerRepository>) -> Self {
        Self { ledger_repo }
    }
}

fn ledger_to_response(ledger: &Ledger) -> LedgerResponse {
    LedgerResponse {
        id: ledger.id.to_string(),
        name: ledger.name.clone(),
        description: ledger.description.clone(),
        ledger_type: ledger.ledger_type.to_string(),
        currency: ledger.currency.clone(),
        created_at: ledger.created_at.to_rfc3339(),
    }
}

pub async fn list_ledgers(
    State(api): State<Arc<LedgerApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Query(params): Query<ListLedgersParams>,
) -> AppResult<Json<ApiResponse<LedgerListResponse>>> {
    let ledgers = if let Some(ref ledger_type) = params.ledger_type {
        if ledger_type == "group" {
            // TODO: Get group ledgers user has access to
            vec![]
        } else {
            api.ledger_repo.find_by_user_id(current_user.id).await?
        }
    } else {
        api.ledger_repo.find_by_user_id(current_user.id).await?
    };

    let items: Vec<LedgerResponse> = ledgers.iter().map(ledger_to_response).collect();

    Ok(Json(ApiResponse::success(LedgerListResponse { items })))
}

pub async fn create_ledger(
    State(api): State<Arc<LedgerApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<CreateLedgerRequest>,
) -> AppResult<Json<ApiResponse<LedgerResponse>>> {
    let ledger = api
        .ledger_repo
        .create(CreateLedger {
            name: req.name,
            description: req.description,
            ledger_type: LedgerType::Personal,
            user_id: Some(current_user.id),
            group_id: None,
            currency: req.currency,
        })
        .await?;

    Ok(Json(ApiResponse::success(ledger_to_response(&ledger))))
}

pub async fn get_ledger(
    State(api): State<Arc<LedgerApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<LedgerResponse>>> {
    let ledger_id = Uuid::parse_str(&id).map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    let ledger = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    // Check ownership
    if ledger.user_id != Some(current_user.id) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    Ok(Json(ApiResponse::success(ledger_to_response(&ledger))))
}

pub async fn update_ledger(
    State(api): State<Arc<LedgerApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateLedgerRequest>,
) -> AppResult<Json<ApiResponse<LedgerResponse>>> {
    let ledger_id = Uuid::parse_str(&id).map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    // Check ownership
    let existing = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if existing.user_id != Some(current_user.id) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    let ledger = api
        .ledger_repo
        .update(ledger_id, req.name, req.description)
        .await?;

    Ok(Json(ApiResponse::success(ledger_to_response(&ledger))))
}

pub async fn delete_ledger(
    State(api): State<Arc<LedgerApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let ledger_id = Uuid::parse_str(&id).map_err(|_| AppError::Validation("Invalid ledger ID".to_string()))?;

    // Check ownership
    let existing = api
        .ledger_repo
        .find_by_id(ledger_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Ledger not found".to_string()))?;

    if existing.user_id != Some(current_user.id) {
        return Err(AppError::Forbidden("Access denied".to_string()));
    }

    api.ledger_repo.delete(ledger_id).await?;

    Ok(Json(ApiResponse::success(())))
}

#[derive(Debug, serde::Deserialize)]
pub struct ListLedgersParams {
    #[serde(rename = "type")]
    pub ledger_type: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateLedgerRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
