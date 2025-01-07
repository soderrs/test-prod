use crate::{friends::Friend, middlewares::authorize::User};
use axum::{response::IntoResponse, Extension, Json};
use sqlx::{sqlite::SqlitePool, Row};
use std::env;

pub async fn remove_friend(
    Extension(user): Extension<User>,
    Json(login): Json<String>,
) -> impl IntoResponse {
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

    let mut user_friends: Vec<Friend> =
        serde_json::from_str(user_friends_row.get("friends")).unwrap_or_default();

    let idx = user_friends
        .iter()
        .rposition(|friend| friend.login == login)
        .unwrap();

    user_friends.remove(idx);
    let user_friends_str = serde_json::to_string(&user_friends).unwrap();

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
    .unwrap();
}
