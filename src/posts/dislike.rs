use super::post_by_id::get_post_by_id;
use crate::middlewares::authorize::User;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension};
use sqlx::SqlitePool;
use std::env;

pub async fn dislike_post(
    Extension(user): Extension<User>,
    Path(post_id): Path<String>,
) -> impl IntoResponse {
    let mut post = match get_post_by_id(Extension(user.clone()), Path(post_id))
        .await
        .0
    {
        Some(post) => post,
        None => {
            return StatusCode::NOT_FOUND;
        }
    };

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if post.disliked_users.contains(&user.login) {
        return StatusCode::OK;
    } else if post.liked_users.contains(&user.login) {
        post.liked_users.remove(&user.login);
        post.likes_count -= 1;
        post.disliked_users.insert(user.login.clone());
        post.dislikes_count += 1;
    } else {
        post.disliked_users.insert(user.login.clone());
        post.dislikes_count += 1;
    }

    sqlx::query(
        r#"
        UPDATE posts
        SET likes_count = ?, dislikes_count = ?, liked_users = ?, disliked_users = ?
        WHERE author = ?
        "#,
    )
    .bind(post.likes_count)
    .bind(post.dislikes_count)
    .bind(post.liked_users)
    .bind(post.disliked_users)
    .bind(post.author)
    .execute(&pool)
    .await
    .unwrap();

    StatusCode::OK
}
