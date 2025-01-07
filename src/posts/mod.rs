use chrono::{DateTime, Utc};

pub mod new;
pub mod post_by_id;
pub mod publish;


pub struct Post {
    pub id: String,
    pub content: String,
    pub author: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub likes_count: u32,
    pub dislikes_count: u32,
}
