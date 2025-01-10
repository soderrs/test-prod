use crate::{
    auth::User,
    middlewares::authorize::{hash_password, verify_password, AppState},
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use axum_extra::{
    extract::TypedHeader,
    headers::{authorization::Bearer, Authorization},
};
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
    State(state): State<AppState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    Json(passwords): Json<UpdatePassword>,
) -> Result<(), StatusCode> {
    if passwords.old_password == passwords.new_password {
        return Err(StatusCode::FORBIDDEN);
    } else if !verify_password(&passwords.old_password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::FORBIDDEN);
    }

    let new_password_hash =
        hash_password(&passwords.new_password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    user.password_hash = new_password_hash;

    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

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

    let token = bearer.token().to_string();

    state
        .revoked_tokens
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .insert(token);

    Ok(())
}
