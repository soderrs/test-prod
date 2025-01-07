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

pub async fn register_user(user: Json<CreateUser>) -> StatusCode {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::query(
        r#"
        INSERT INTO users VALUES (?, ?, ?, ?, ?, ?, ?, ?);
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
    .unwrap();

    StatusCode::CREATED
}
