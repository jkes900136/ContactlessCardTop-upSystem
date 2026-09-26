use serde::{Deserialize, Serialize};
use sqlx::types::Decimal;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Card {
    pub id: Uuid,
    pub card_uid: String,
    pub balance: Decimal,
    pub status: String, // "active", "disabled", "lost"
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub card_uid: String,
    pub amount: Decimal,
    pub timestamp: NaiveDateTime,
}
