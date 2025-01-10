use axum::{extract::Path, http::StatusCode, Json};
use serde::Serialize;
use sqlx::{prelude::FromRow, sqlite::SqlitePool};
use std::env;

#[derive(Serialize, Clone, FromRow, Debug)]
pub struct Country {
    name: String,
    alpha2: String,
    alpha3: String,
    region: String,
}

pub async fn countries() -> Result<Json<Vec<Country>>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let countries: Vec<Country> = sqlx::query_as("SELECT * FROM countries;")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(Json::from(countries))
}

pub async fn country_by_id(alpha2: Path<String>) -> Result<Json<Country>, StatusCode> {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let country: Country = sqlx::query_as(
        r#"
        SELECT * FROM countries WHERE alpha2 = ?
        "#,
    )
    .bind(alpha2.to_string())
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json::from(country))
}
