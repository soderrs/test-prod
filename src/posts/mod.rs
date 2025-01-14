use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

pub mod dislike;
pub mod feed;
pub mod like;
pub mod new;
pub mod post_by_id;

#[derive(Deserialize)]
pub struct CreatePost {
    pub content: String,
    pub tags: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Post {
    pub id: String,
    pub content: String,
    pub author: String,
    pub tags: Json<HashSet<String>>,
    pub created_at: String,
    pub likes_count: u32,
    pub dislikes_count: u32,
    pub liked_users: Json<HashSet<String>>,
    pub disliked_users: Json<HashSet<String>>,
}
