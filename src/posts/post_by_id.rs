use crate::{auth::User, friends::Friend, posts::Post};
use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::SqlitePool;
use std::env;

pub async fn get_post_by_id(
    Extension(user): Extension<User>,
    Path(post_id): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let post: Post = sqlx::query_as(
        r#"
        SELECT * FROM posts
        WHERE id = ?
        "#,
    )
    .bind(post_id)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    let author_user: User = sqlx::query_as(
        r#"
        SELECT * FROM users WHERE login = ?
        "#,
    )
    .bind(&post.author)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let author_friends: Vec<Friend> = author_user.friends.unwrap_or(sqlx::types::Json(vec![])).0;

    if author_friends
        .iter()
        .find(|friend| &friend.login == &user.login)
        .is_some()
    {
        return Ok(Json(post));
    } else if author_user.is_public {
        Ok(Json(post))
    } else if user.login == author_user.login {
        Ok(Json(post))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
