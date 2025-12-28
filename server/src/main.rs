use std::net::SocketAddr;
use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use money_notes_server::api::{
    create_routes, routes::AppState, AuthApi, BillApi, CategoryApi, GroupApi, LedgerApi, UserApi,
};
use money_notes_server::config::Settings;
use money_notes_server::middleware::AuthState;
use money_notes_server::repositories::mysql::{
    MySqlBillRepository, MySqlCategoryRepository, MySqlGroupRepository, MySqlLedgerRepository,
    MySqlUserRepository,
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

    // Load configuration
    dotenvy::dotenv().ok();
    let settings = Settings::new().unwrap_or_else(|e| {
        tracing::warn!("Failed to load config: {}, using defaults", e);
        Settings::default()
    });

    tracing::info!("Starting server with {} database", settings.database.driver);

    // Initialize database connection
    let pool = sqlx::MySqlPool::connect(&settings.database.url)
        .await
        .expect("Failed to connect to database");

    // Initialize repositories
    let user_repo: Arc<dyn money_notes_server::repositories::UserRepository> =
        Arc::new(MySqlUserRepository::new(pool.clone()));
    let ledger_repo: Arc<dyn money_notes_server::repositories::LedgerRepository> =
        Arc::new(MySqlLedgerRepository::new(pool.clone()));
    let bill_repo: Arc<dyn money_notes_server::repositories::BillRepository> =
        Arc::new(MySqlBillRepository::new(pool.clone()));
    let category_repo: Arc<dyn money_notes_server::repositories::CategoryRepository> =
        Arc::new(MySqlCategoryRepository::new(pool.clone()));
    let group_repo: Arc<dyn money_notes_server::repositories::GroupRepository> =
        Arc::new(MySqlGroupRepository::new(pool.clone()));

    // Initialize default categories
    if let Err(e) = category_repo.init_default_categories().await {
        tracing::warn!("Failed to init default categories: {}", e);
    }

    // Initialize JWT util
    let jwt_util = JwtUtil::new(
        settings.jwt.secret.clone(),
        settings.jwt.access_token_expires,
        settings.jwt.refresh_token_expires,
    );

    // Initialize services
    let auth_service = Arc::new(AuthService::new(user_repo.clone(), jwt_util.clone()));

    // Initialize API handlers
    let auth_api = Arc::new(AuthApi::new(auth_service, settings.jwt.access_token_expires));
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

    let app = create_routes(app_state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let addr = SocketAddr::from((
        settings.server.host.parse::<std::net::IpAddr>().unwrap(),
        settings.server.port,
    ));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
