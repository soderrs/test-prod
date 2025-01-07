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

pub async fn countries() -> (StatusCode, Json<Vec<Country>>) {
    let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let countries: Vec<Country> = sqlx::query_as("SELECT * FROM countries;")
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json::from(countries))
}

pub async fn country_by_id(alpha2: Path<String>) -> (StatusCode, Json<Country>) {
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
    .unwrap();
    (StatusCode::OK, Json::from(country))
}
