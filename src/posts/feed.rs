use super::Post;
use crate::auth::User;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::SqlitePool;
use std::env;

pub async fn feed_my(Extension(user): Extension<User>) -> Result<Json<Vec<Post>>, StatusCode> {
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts WHERE author = ?
        "#,
    )
    .bind(&user.login)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

pub async fn feed_user(
    Extension(user): Extension<User>,
    Path(login): Path<String>,
) -> Result<Json<Option<Vec<Post>>>, StatusCode> {
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_from_login: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE login = ?
        "#,
    )
    .bind(&login)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

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
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(Some(posts)))
    } else {
        Ok(Json(None))
    }
}
