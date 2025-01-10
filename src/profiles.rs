use crate::{auth::User, friends::Friend};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Row, SqlitePool};
use std::env;

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub login: String,
    pub email: String,
    pub country_code: String,
    pub is_public: bool,
    pub phone: Option<String>,
    pub image: Option<String>,
}

pub async fn profile_by_login(
    login: Path<String>,
    Extension(user): Extension<User>,
) -> impl IntoResponse {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let user_profile: Option<UserProfile> = if user.login.to_string() == login.to_string() {
        Some(UserProfile {
            login: user.login,
            email: user.email,
            country_code: user.country_code,
            is_public: user.is_public,
            phone: user.phone,
            image: user.image,
        })
    } else {
        let get_user = sqlx::query(
            r#"
        SELECT * FROM users WHERE login = ?
        "#,
        )
        .bind(login.to_string())
        .fetch_optional(&pool)
        .await
        .unwrap()
        .unwrap();

        let get_user_friends: Vec<Friend> =
            serde_json::from_str(get_user.get("friends")).unwrap_or_default();

        if get_user_friends
            .iter()
            .find(|friend| friend.login == user.login)
            .is_some()
            || get_user.get("is_public")
        {
            return Json(Some(UserProfile {
                login: get_user.get("login"),
                email: get_user.get("email"),
                country_code: get_user.get("country_code"),
                is_public: get_user.get("is_public"),
                phone: get_user.get("phone"),
                image: get_user.get("image"),
            }));
        } else {
            None
        }
    };

    if user_profile.is_some() {
        Json(user_profile)
    } else {
        Json(None)
    }
}
