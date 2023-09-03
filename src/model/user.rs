use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::debug_handler;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::errors::CustomError;


#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct UpdateUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

pub async fn all_users(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM users ".to_string();

    let user = sqlx::query_as::<_, User>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(user))
}

pub async fn get_user(
    Path(id):Path<i32>,
    Extension(pool): Extension<PgPool>
) -> Result<Json<User>, CustomError> {
    let sql = "SELECT * FROM users WHERE id=$1".to_string();

    let user: User = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::UserNotFound
        })?;

    Ok(Json(user))  
}

#[debug_handler]
pub async fn new_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<NewUser>
) -> Result<(StatusCode, Json<NewUser>), CustomError> {
    if user.first_name.is_empty() || user.last_name.is_empty() || user.email.is_empty() {
        return Err(CustomError::BadRequest)
    }
    let sql = "INSERT INTO users (first_name, last_name, email) VALUES ($1, $2, $3)";

    let _ = sqlx::query(&sql)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .execute(&pool)
        .await
        .map_err(|_| {
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(user): Json<UpdateUser>
) -> Result<(StatusCode, Json<UpdateUser>), CustomError> {
    if user.first_name.is_empty() || user.last_name.is_empty() || user.email.is_empty() {
        return Err(CustomError::BadRequest)
    }
    let sql = "SELECT * FROM user WHERE id=$1".to_string();

    let _user: User = sqlx::query_as(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::UserNotFound
        })?;

    let _ = sqlx::query("UPDATE user SET first_name=$1, last_name=$2, email=$3 WHERE id=$4")
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(id)
        .execute(&pool)
        .await;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user(
    Path(id): Path<i32>, 
    Extension(pool): Extension<PgPool>
) -> Result<(StatusCode, Json<Value>), CustomError> {


    let _find: User = sqlx::query_as("SELECT * FROM user WHERE id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            CustomError::UserNotFound
        })?;

    sqlx::query("DELETE FROM user WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| {
            CustomError::UserNotFound
        })?;

        Ok((StatusCode::OK, Json(json!({"msg": "User Deleted"}))))
}
