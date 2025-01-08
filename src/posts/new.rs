use crate::{middlewares::authorize::User, posts::CreatePost, posts::Post};
use axum::{response::IntoResponse, Extension, Json};
use chrono::Utc;
use sqlx::sqlite::SqlitePool;
use std::env;

pub async fn new_post(
    Extension(user): Extension<User>,
    Json(create_post): Json<CreatePost>,
) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_posts: Vec<Post> = sqlx::query_as(
        r#"
        SELECT * FROM posts WHERE author = ?
        "#,
    )
    .bind(&user.login)
    .fetch_all(&pool)
    .await
    .unwrap();

    let post = Post {
        id: format!("{}_{}", &user.login, user_posts.len()),
        content: create_post.content,
        author: user.login.clone(),
        tags: sqlx::types::Json(create_post.tags.clone()),
        created_at: Utc::now().to_string(),
        likes_count: 0,
        dislikes_count: 0,
    };

    sqlx::query(
        r#"
        INSERT INTO posts (id, content, author, tags, created_at, likes_count, dislikes_count)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(post.id)
    .bind(post.content)
    .bind(post.author)
    .bind(serde_json::to_string(&post.tags).unwrap())
    .bind(post.created_at.to_string())
    .bind(post.likes_count)
    .bind(post.dislikes_count)
    .execute(&pool)
    .await
    .unwrap();
}
