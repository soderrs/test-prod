use crate::middlewares::authorize::User;
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::env;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub login: String,
    pub email: String,
    pub country_code: String,
    pub is_public: bool,
    pub phone: Option<String>,
    pub image: Option<String>,
}

pub async fn profile_by_login(
    login: Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_profile: Option<UserProfile> = if user.login.to_string() == login.to_string() {
        Some(UserProfile {
            login: user.login,
            email: user.email,
            country_code: user.country_code,
            is_public: user.is_public,
            phone: user.phone,
            image: user.image,
        })
    } else {
        sqlx::query_as(
            r#"
        SELECT * FROM users WHERE login = ? AND is_public = 1
        "#,
        )
        .bind(login.to_string())
        .fetch_optional(&pool)
        .await
        .unwrap()
    };

    Json(user_profile)
}
