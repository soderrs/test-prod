use crate::{auth::User, posts::CreatePost, posts::Post};
use axum::{http::StatusCode, Extension, Json};
use chrono::Utc;
use sqlx::sqlite::SqlitePool;
use std::{collections::HashSet, env};

pub async fn new_post(
    Extension(user): Extension<User>,
    Json(create_post): Json<CreatePost>,
) -> Result<StatusCode, StatusCode> {
    let pool = SqlitePool::connect(
        &env::var("DATABASE_URL").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts WHERE author = ?
        "#,
    )
    .bind(&user.login)
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post = Post {
        id: format!("{}_{}", &user.login, user_posts.len()),
        content: create_post.content,
        author: user.login.clone(),
        tags: sqlx::types::Json(create_post.tags.clone()),
        created_at: Utc::now().to_string(),
        likes_count: 0,
        dislikes_count: 0,
        liked_users: sqlx::types::Json(HashSet::new()),
        disliked_users: sqlx::types::Json(HashSet::new()),
    };

    sqlx::query(
        r#"
        INSERT INTO posts
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(post.id)
    .bind(post.content)
    .bind(post.author)
    .bind(serde_json::to_string(&post.tags).unwrap())
    .bind(post.created_at.to_string())
    .bind(post.likes_count)
    .bind(post.dislikes_count)
    .bind(post.liked_users)
    .bind(post.disliked_users)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::CREATED)
}
