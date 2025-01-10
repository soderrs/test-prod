use crate::friends::Friend;
use serde::{Deserialize, Serialize};
use sqlx::{types::Json, FromRow};

pub mod register;
pub mod sign_in;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub email: String,
    pub country_code: String,
    pub password_hash: String,
    pub is_public: bool,
    pub phone: Option<String>,
    pub image: Option<String>,
    pub friends: Option<sqlx::types::Json<Vec<Friend>>>,
    pub posts: Option<Json<Vec<String>>>,
}
