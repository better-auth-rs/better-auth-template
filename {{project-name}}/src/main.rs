use axum::{Router, routing::get};
{% if database == "postgres" %}use better_auth::adapters::SqlxAdapter;
{% else %}use better_auth::adapters::MemoryDatabaseAdapter;
{% endif %}use better_auth::handlers::AxumIntegration;
use better_auth::plugins::{
    EmailPasswordPlugin, PasswordManagementPlugin, SessionManagementPlugin,
};
use better_auth::{AuthBuilder, AuthConfig, BetterAuth};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

{% if database == "postgres" %}type DB = SqlxAdapter;
{% else %}type DB = MemoryDatabaseAdapter;
{% endif %}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = AuthConfig::new("your-very-secure-secret-key-at-least-32-chars-long")
        .base_url("http://localhost:3000")
        .password_min_length(8);
{% if database == "postgres" %}
    dotenvy::dotenv().ok();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database = SqlxAdapter::new(&database_url).await?;
{% else %}
    let database = MemoryDatabaseAdapter::new();
{% endif %}
    let auth: Arc<BetterAuth<DB>> = Arc::new(
        AuthBuilder::new(config)
            .database(database)
            .plugin(EmailPasswordPlugin::new().enable_signup(true))
            .plugin(SessionManagementPlugin::new())
            .plugin(PasswordManagementPlugin::new())
            .build()
            .await?,
    );

    println!("Registered plugins: {:?}", auth.plugin_names());

    let auth_router = auth.clone().axum_router();

    let app = Router::new()
        .route("/", get(|| async { "Hello from {{project-name}}!" }))
        .nest("/auth", auth_router)
        .layer(CorsLayer::permissive())
        .with_state(auth);

    println!("Listening on http://localhost:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
