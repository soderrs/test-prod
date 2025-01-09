use super::Post;
use crate::middlewares::authorize::User;
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;
use std::env;

pub async fn feed_my(Extension(user): Extension<User>) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts WHERE author = ?
        "#,
    )
    .bind(&user.login)
    .fetch_all(&pool)
    .await
    .unwrap();

    Json(posts)
}

pub async fn feed_user(
    Extension(user): Extension<User>,
    Path(login): Path<String>,
) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_from_login: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE login = ?
        "#,
    )
    .bind(&login)
    .fetch_one(&pool)
    .await
    .unwrap();

    if user_from_login.is_public
        || user_from_login
            .friends
            .unwrap_or(sqlx::types::Json(vec![]))
            .iter()
            .find(|friend| friend.login == user.login)
            .is_some()
    {
        let posts: Vec<Post> = sqlx::query_as(
            r#"
            SELECT * FROM posts WHERE author = ?
            "#,
        )
        .bind(login)
        .fetch_all(&pool)
        .await
        .unwrap();

        Json(Some(posts))
    } else {
        Json(None)
    }
}
