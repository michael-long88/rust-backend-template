use std::net::SocketAddr;
use std::env;

use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    response::IntoResponse,
    extract::Path,
    Json, Router, Extension
};

use errors::CustomError;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use anyhow::Result;
use tower_http::trace::TraceLayer;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod model;
mod util;
mod errors;

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let database_url = util::get_database_url();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .map_err(|_| {
            CustomError::DatabaseError
        })?;

    sqlx::migrate!().run(&pool).await.map_err(|_| CustomError::MigrationError)?;

    if let Ok(env_mode) = env::var("ENV_MODE") {
        if env_mode == "dev" {
            util::seed_users(&pool).await?;
        }
    }

    let app = Router::new()
        .route("/", get(root))
        .route("/hello/:name", get(json_hello))
        .route("/users", get(model::user::all_users))
        .route("/user", post(model::user::new_user))
        .route("/user/:id", get(model::user::get_user))
        .route("/user/:id", put(model::user::update_user))
        .route("/user/:id", delete(model::user::delete_user))
        .layer(Extension(pool))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

        Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn json_hello(Path(name): Path<String>) -> impl IntoResponse { 
    let hello = String::from("Hello ");
    let greeting = name.as_str();

    (StatusCode::OK, Json(json!({"message": hello + greeting }))) 
}
