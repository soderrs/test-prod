use axum::{
    body::Body,
    extract::{Json, Request, State},
    http::{self, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{prelude::FromRow, sqlite::SqlitePool};
use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    login: String,
}

pub struct AuthError {
    pub message: String,
    pub status_code: StatusCode,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}

pub fn encode_jwt(login: String) -> Result<String, StatusCode> {
    let secret = "random_string_typically_from_env".to_string();
    let now = Utc::now();
    let expire = Duration::hours(1);
    let exp = (now + expire).timestamp() as usize;
    let iat = now.timestamp() as usize;
    let claims = Claims { iat, exp, login };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret = "random_string_typically_from_env".to_string();

    decode(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Clone)]
pub struct AppState {
    pub revoked_tokens: Arc<Mutex<HashSet<String>>>,
}

pub async fn authorize_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, AuthError> {
    let auth_header = if let Some(header) = req.headers_mut().get(http::header::AUTHORIZATION) {
        header.to_str().map_err(|_| AuthError {
            message: "Empty header is not allowed".to_string(),
            status_code: StatusCode::FORBIDDEN,
        })?
    } else {
        return Err(AuthError {
            message: "Please add the JWT to the header".to_string(),
            status_code: StatusCode::FORBIDDEN,
        });
    };

    let [_, token, ..] = *auth_header.split_whitespace().collect::<Vec<&str>>() else {
        return Err(AuthError {
            message: "Error parsing header".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        });
    };

    let token_data = match decode_jwt(token.to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(AuthError {
                message: "Unable to decode token".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    if state.revoked_tokens.lock().unwrap().contains(token) {
        return Err(AuthError {
            message: "Please sign in".to_string(),
            status_code: StatusCode::UNAUTHORIZED,
        });
    }

    let user = match retrieve_user_by_login(&token_data.claims.login).await {
        Some(user) => user,
        None => {
            return Err(AuthError {
                message: "You are not an authorized user".to_string(),
                status_code: StatusCode::UNAUTHORIZED,
            });
        }
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

#[derive(Debug, FromRow, Clone, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub email: String,
    pub country_code: String,
    pub password_hash: String,
    pub is_public: bool,
    pub phone: Option<String>,
    pub image: Option<String>,
}

pub async fn retrieve_user_by_login(login: &str) -> Option<User> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    Some(
        sqlx::query_as(
            r#"
        SELECT * FROM users WHERE login = ?
        "#,
        )
        .bind(login)
        .fetch_one(&pool)
        .await
        .unwrap(),
    )
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}
