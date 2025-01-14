use crate::{auth::User, profiles::UserProfile};
use axum::{http::StatusCode, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqlitePool};
use std::env;

pub async fn get_profile(Extension(user): Extension<User>) -> Json<UserProfile> {
    let user_profile = UserProfile {
        login: user.login,
        email: user.email,
        country_code: user.country_code,
        is_public: user.is_public,
        phone: user.phone,
        image: user.image,
    };
    Json(user_profile)
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UpdateUser {
    pub login: Option<String>,
    pub email: Option<String>,
    pub country_code: Option<String>,
    pub password_hash: Option<String>,
    pub is_public: Option<bool>,
    pub phone: Option<String>,
    pub image: Option<String>,
}

pub async fn update_profile(
    Extension(mut old_user): Extension<User>,
    new_user: Json<UpdateUser>,
) -> Result<Json<UserProfile>, StatusCode> {
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let old_login = old_user.login.clone();
    old_user.login = new_user.login.clone().unwrap_or(old_user.login);
    old_user.email = new_user.email.clone().unwrap_or(old_user.email);
    old_user.is_public = new_user.is_public.unwrap_or(old_user.is_public);
    old_user.phone = if new_user.phone.is_some() {
        new_user.phone.clone()
    } else {
        old_user.phone
    };
    old_user.image = if new_user.image.is_some() {
        new_user.image.clone()
    } else {
        old_user.image
    };

    sqlx::query(
        r#"
        UPDATE users
        SET login = ?, email = ?, is_public = ?, phone = ?, image = ?
        WHERE login = ?
        "#,
    )
    .bind(&old_user.login)
    .bind(&old_user.email)
    .bind(old_user.is_public)
    .bind(&old_user.phone)
    .bind(&old_user.image)
    .bind(old_login)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_profile = UserProfile {
        login: old_user.login,
        email: old_user.email,
        country_code: old_user.country_code,
        is_public: old_user.is_public,
        phone: old_user.phone,
        image: old_user.image,
    };

    Ok(Json(user_profile))
}
