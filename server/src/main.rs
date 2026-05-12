use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio::fs;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod auth;
mod errors;
mod files;
mod state;

use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let storage_path =
        std::env::var("STORAGE_PATH").unwrap_or_else(|_| "./data/files".to_string());
    let port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    tracing::info!("Migrations applied");

    fs::create_dir_all(&storage_path).await?;
    tracing::info!("Storage path: {storage_path}");

    let state = Arc::new(AppState {
        pool,
        jwt_secret,
        storage_path,
    });

    let app = Router::new()
        .nest("/api/auth", auth::router(state.clone()))
        .nest("/api/files", files::router(state.clone()))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive()); // Restrict to known origins in prod

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
