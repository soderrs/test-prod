use crate::middlewares::authorize::{hash_password, verify_password, User};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::env;

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub old_password: String,
    pub new_password: String,
}

pub async fn update_password(
    Extension(mut user): Extension<User>,
    passwords: Json<UpdatePassword>,
) -> impl IntoResponse {
    if passwords.old_password == passwords.new_password {
        return Err(StatusCode::FORBIDDEN);
    } else if !verify_password(&passwords.old_password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let new_password_hash =
        hash_password(&passwords.new_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    user.password_hash = new_password_hash;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::query(
        r#"
        UPDATE users
        SET password_hash = ?
        WHERE login = ?
        "#,
    )
    .bind(user.password_hash)
    .bind(user.login)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
