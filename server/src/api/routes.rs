use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

use crate::api::auth::{login, refresh, register, AuthApi};
use crate::api::bill::{create_bill, delete_bill, get_bill, get_category_statistics, get_statistics, get_trend_statistics, list_bills, update_bill, BillApi};
use crate::api::category::{create_category, delete_category, list_categories, update_category, CategoryApi};
use crate::api::group::{
    create_group, delete_group, get_group, join_group, leave_group, list_groups,
    remove_member, reset_invite_code, transfer_group, update_group, update_member_role, GroupApi,
};
use crate::api::ledger::{create_ledger, delete_ledger, get_ledger, list_ledgers, update_ledger, LedgerApi};
use crate::api::user::{change_password, get_me, update_me, UserApi};
use crate::middleware::{auth_middleware, AuthState};

pub struct AppState {
    pub auth_api: Arc<AuthApi>,
    pub user_api: Arc<UserApi>,
    pub ledger_api: Arc<LedgerApi>,
    pub bill_api: Arc<BillApi>,
    pub category_api: Arc<CategoryApi>,
    pub group_api: Arc<GroupApi>,
    pub auth_state: AuthState,
}

pub fn create_routes(state: AppState) -> Router {
    // Public routes (no auth required)
    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .with_state(state.auth_api.clone());

    let public_category_routes = Router::new()
        .route("/", get(list_categories))
        .with_state(state.category_api.clone());

    // Protected routes (auth required)
    let user_routes = Router::new()
        .route("/me", get(get_me))
        .route("/me", put(update_me))
        .route("/me/password", put(change_password))
        .with_state(state.user_api.clone())
        .layer(middleware::from_fn_with_state(
            state.auth_state.clone(),
            auth_middleware,
        ));

    let ledger_routes = Router::new()
        .route("/", get(list_ledgers))
        .route("/", post(create_ledger))
        .route("/:id", get(get_ledger))
        .route("/:id", put(update_ledger))
        .route("/:id", delete(delete_ledger))
        .with_state(state.ledger_api.clone())
        .layer(middleware::from_fn_with_state(
            state.auth_state.clone(),
            auth_middleware,
        ));

    let bill_routes = Router::new()
        .route("/", get(list_bills))
        .route("/", post(create_bill))
        .route("/statistics", get(get_statistics))
        .route("/statistics/category", get(get_category_statistics))
        .route("/statistics/trend", get(get_trend_statistics))
        .route("/:id", get(get_bill))
        .route("/:id", put(update_bill))
        .route("/:id", delete(delete_bill))
        .with_state(state.bill_api.clone())
        .layer(middleware::from_fn_with_state(
            state.auth_state.clone(),
            auth_middleware,
        ));

    let protected_category_routes = Router::new()
        .route("/", post(create_category))
        .route("/:id", put(update_category))
        .route("/:id", delete(delete_category))
        .with_state(state.category_api.clone())
        .layer(middleware::from_fn_with_state(
            state.auth_state.clone(),
            auth_middleware,
        ));

    let group_routes = Router::new()
        .route("/", get(list_groups))
        .route("/", post(create_group))
        .route("/join", post(join_group))
        .route("/:id", get(get_group))
        .route("/:id", put(update_group))
        .route("/:id", delete(delete_group))
        .route("/:id/leave", post(leave_group))
        .route("/:id/transfer", post(transfer_group))
        .route("/:id/invite-code", post(reset_invite_code))
        .route("/:group_id/members/:user_id", delete(remove_member))
        .route("/:group_id/members/:user_id/role", put(update_member_role))
        .with_state(state.group_api.clone())
        .layer(middleware::from_fn_with_state(
            state.auth_state.clone(),
            auth_middleware,
        ));

    Router::new()
        .nest("/api/v1/auth", auth_routes)
        .nest("/api/v1/users", user_routes)
        .nest("/api/v1/ledgers", ledger_routes)
        .nest("/api/v1/bills", bill_routes)
        .nest("/api/v1/categories", public_category_routes.merge(protected_category_routes))
        .nest("/api/v1/groups", group_routes)
}
