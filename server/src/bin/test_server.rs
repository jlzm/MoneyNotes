use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use money_notes_server::api::{
    create_routes, routes::AppState, AuthApi, BillApi, CategoryApi, GroupApi, LedgerApi, UserApi,
};
use money_notes_server::middleware::AuthState;
use money_notes_server::repositories::memory::{
    MemoryBillRepository, MemoryCategoryRepository, MemoryGroupRepository, MemoryLedgerRepository,
    MemoryUserRepository,
};
use money_notes_server::services::AuthService;
use money_notes_server::utils::JwtUtil;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "money_notes_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Money Notes server with in-memory storage (for testing)");

    // Initialize in-memory repositories
    let user_repo: Arc<dyn money_notes_server::repositories::UserRepository> =
        Arc::new(MemoryUserRepository::new());
    let ledger_repo: Arc<dyn money_notes_server::repositories::LedgerRepository> =
        Arc::new(MemoryLedgerRepository::new());
    let bill_repo: Arc<dyn money_notes_server::repositories::BillRepository> =
        Arc::new(MemoryBillRepository::new());
    let category_repo: Arc<dyn money_notes_server::repositories::CategoryRepository> =
        Arc::new(MemoryCategoryRepository::new());
    let group_repo: Arc<dyn money_notes_server::repositories::GroupRepository> =
        Arc::new(MemoryGroupRepository::new());

    // Initialize default categories
    if let Err(e) = category_repo.init_default_categories().await {
        tracing::warn!("Failed to init default categories: {}", e);
    }

    // Initialize JWT util
    let jwt_util = JwtUtil::new(
        "test-secret-key-for-development".to_string(),
        3600,   // 1 hour
        604800, // 7 days
    );

    // Initialize services
    let auth_service = Arc::new(AuthService::new(user_repo.clone(), jwt_util.clone()));

    // Initialize API handlers
    let auth_api = Arc::new(AuthApi::new(auth_service, 3600));
    let user_api = Arc::new(UserApi::new(user_repo.clone()));
    let ledger_api = Arc::new(LedgerApi::new(ledger_repo.clone()));
    let bill_api = Arc::new(BillApi::new(
        bill_repo.clone(),
        ledger_repo.clone(),
        category_repo.clone(),
        user_repo.clone(),
    ));
    let category_api = Arc::new(CategoryApi::new(category_repo.clone()));
    let group_api = Arc::new(GroupApi::new(group_repo.clone(), user_repo.clone(), ledger_repo.clone()));

    let auth_state = AuthState { jwt_util };

    // Build router
    let app_state = AppState {
        auth_api,
        user_api,
        ledger_api,
        bill_api,
        category_api,
        group_api,
        auth_state,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let app = create_routes(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server listening on http://{}", addr);
    tracing::info!("API endpoints available at http://{}/api/v1/", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
