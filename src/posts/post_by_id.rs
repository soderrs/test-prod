use crate::middlewares::authorize::User;
use crate::posts::Post;
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use sqlx::SqlitePool;
use std::env;

pub async fn get_post_by_id(
    Extension(_user): Extension<User>,
    Path(post_id): Path<String>,
) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let post: Post = sqlx::query_as(
        r#"
        SELECT * FROM posts
        "#,
    )
    .bind(post_id)
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(post)
}
