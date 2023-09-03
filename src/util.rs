use std::env;

use anyhow::Result;
use sqlx::PgPool;

use crate::{model::user::User, errors::CustomError};

pub fn get_database_url() -> String {
    let username = env::var("DB_USER").expect("DB_USER must be set");
    let password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");
    let port = env::var("DB_PORT").expect("DB_PORT must be set");

    format!(
        "postgresql://{}:{}@{}:{}/{}",
        username, password, host, port, db_name
    )
}

pub async fn seed_users(pool: &PgPool) -> Result<(), CustomError> {
    let sql = "INSERT INTO users (id, first_name, last_name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING";
    let user1 = User {
        id: 1,
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: "john_doe@email.com".to_string(),
    };
    let user2 = User {
        id: 2,
        first_name: "Jane".to_string(),
        last_name: "Doe".to_string(),
        email: "jane_doe@email.com".to_string(),
    };

    for user in vec![user1, user2] {
        sqlx::query(&sql)
            .bind(&user.id)
            .bind(&user.first_name)
            .bind(&user.last_name)
            .bind(&user.email)
            .execute(pool)
            .await
            .map_err(|_| {
                CustomError::InternalServerError
            })?;
    }

    Ok(())
}