use crate::middlewares::authorize::{encode_jwt, retrieve_user_by_login};
use axum::{extract::Json, http::StatusCode};
use bcrypt::verify;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInData {
    pub login: String,
    pub password: String,
}

pub async fn sign_in(Json(sign_in_data): Json<SignInData>) -> Result<Json<String>, StatusCode> {
    let user = match retrieve_user_by_login(&sign_in_data.login).await {
        Some(user) => user,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if !verify(&sign_in_data.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = encode_jwt(user.login).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(token))
}
