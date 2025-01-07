use crate::{friends::Friend, middlewares::authorize::User};
use axum::{response::IntoResponse, Extension, Json};
use sqlx::{sqlite::SqlitePool, Row};
use std::env;

pub async fn list_friends(Extension(user): Extension<User>) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_friends_row = sqlx::query(
        r#"
        SELECT friends FROM users WHERE login = ?
        "#,
    )
    .bind(&user.login)
    .fetch_optional(&pool)
    .await
    .unwrap()
    .unwrap();

    let user_friends: Vec<Friend> =
        serde_json::from_str(user_friends_row.get("friends")).unwrap_or_default();

    Json(user_friends)
}
