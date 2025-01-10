use crate::{auth::User, friends::Friend};
use axum::{http::StatusCode, Extension, Json};
use sqlx::{sqlite::SqlitePool, Row};
use std::env;

pub async fn remove_friend(
    Extension(user): Extension<User>,
    Json(login): Json<String>,
) -> Result<(), StatusCode> {
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

    let mut user_friends: Vec<Friend> =
        serde_json::from_str(user_friends_row.get("friends")).unwrap_or_default();

    let idx = if let Some(i) = user_friends
        .iter()
        .rposition(|friend| friend.login == login)
    {
        i
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    user_friends.remove(idx);
    let user_friends_str =
        serde_json::to_string(&user_friends).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query(
        r#"
        UPDATE users
        SET friends = ?
        WHERE login = ?
        "#,
    )
    .bind(user_friends_str)
    .bind(user.login)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
