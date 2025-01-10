use crate::{auth::User, friends::Friend};
use axum::{http::StatusCode, Extension, Json};
use sqlx::{sqlite::SqlitePool, Row};
use std::env;

pub async fn list_friends(
    Extension(user): Extension<User>,
) -> Result<Json<Vec<Friend>>, StatusCode> {
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_friends_row = sqlx::query(
        r#"
        SELECT friends FROM users WHERE login = ?
        "#,
    )
    .bind(&user.login)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_friends: Vec<Friend> =
        serde_json::from_str(user_friends_row.get("friends")).unwrap_or_default();

    Ok(Json(user_friends))
}
