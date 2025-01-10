use super::post_by_id::get_post_by_id;
use crate::auth::User;
use axum::{extract::Path, http::StatusCode, Extension};
use sqlx::SqlitePool;
use std::env;

pub async fn like_post(
    Extension(user): Extension<User>,
    Path(post_id): Path<String>,
) -> Result<(), StatusCode> {
    let mut post = get_post_by_id(Extension(user.clone()), Path(post_id)).await?;

    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    if post.liked_users.contains(&user.login) {
        return Ok(());
    } else if post.disliked_users.contains(&user.login) {
        post.disliked_users.remove(&user.login);
        post.dislikes_count -= 1;
        post.liked_users.insert(user.login.clone());
        post.likes_count += 1;
    } else {
        post.liked_users.insert(user.login.clone());
        post.likes_count += 1;
    }

    sqlx::query(
        r#"
        UPDATE posts
        SET likes_count = ?, dislikes_count = ?, liked_users = ?, disliked_users = ?
        WHERE id = ?
        "#,
    )
    .bind(post.likes_count)
    .bind(post.dislikes_count)
    .bind(&post.liked_users)
    .bind(&post.disliked_users)
    .bind(&post.id)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(())
}
