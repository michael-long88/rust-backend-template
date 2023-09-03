use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;


#[derive(Debug)]
pub enum CustomError {
    BadRequest,
    UserNotFound,
    InternalServerError,
    DatabaseError,
    MigrationError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ),
            Self::BadRequest=> (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
            Self::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Could not connect to database"),
            Self::MigrationError => (StatusCode::INTERNAL_SERVER_ERROR, "Could not migrate database"),
        };
        (status, Json(json!({"error": error_message}))).into_response()
    }
}
