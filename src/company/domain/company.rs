use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Company {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub email: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
}