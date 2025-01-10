use crate::middlewares::authorize::hash_password;
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePool};
use std::env;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub login: String,
    pub email: String,
    pub country_code: String,
    pub password: String,
    pub is_public: bool,
    pub phone: Option<String>,
    pub image: Option<String>,
}

pub async fn register_user(user: Json<CreateUser>) -> Result<(), StatusCode> {
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"
        INSERT INTO users VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
    )
    .bind(&user.login)
    .bind(&user.email)
    .bind(&user.country_code)
    .bind(hash_password(&user.password).unwrap())
    .bind(user.is_public)
    .bind(&user.phone)
    .bind(&user.image)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
