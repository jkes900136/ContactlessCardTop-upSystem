use axum::Json;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use crate::models::{Card, Transaction};

#[derive(Deserialize)]
pub struct CardInfoQuery {
    pub card_uid: String,
}

#[derive(Deserialize)]
pub struct TopupRequest {
    pub card_uid: String,
    pub amount: serde_json::Decimal,
}

pub async fn get_card_info(Query(query): Query<CardInfoQuery>) -> Json<Card> {
    // TODO: Fetch card from database using query.card_uid
    // Placeholder logic
    Json(Card {
        id: uuid::Uuid::new_v4(),
        card_uid: query.card_uid,
        balance: serde_json::Decimal::from(100),
        status: "active".to_string(),
        created_at: chrono::NaiveDateTime::from_timestamp_opt(1600000000, 0).unwrap(),
    })
}

pub async fn post_topup(Json(payload): Json<TopupRequest>) -> Json<serde_json::_Result<String>> {
    // TODO: Process top-up in a transaction
    // 1. Find card
    // 2. Add amount
    // 3. Record transaction
    Json(Ok(format!("Successfully topped up {} for card {}", payload.amount, payload.card_uid)))
}

pub async fn get_transactions() -> Json<Vec<Transaction>> {
    // TODO: Fetch all transactions from database
    Json(vec![])
}

pub async fn get_status() -> Json<serde_json::_Result<String>> {
    Json(Ok("System is healthy".to_string()))
}
