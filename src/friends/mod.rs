use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub mod add;
pub mod list;
pub mod remove;

#[derive(Serialize, Deserialize, Clone, FromRow, PartialEq, Debug)]
pub struct Friend {
    pub login: String,
    pub added_at: DateTime<Utc>,
}
