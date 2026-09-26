use axum::Json;
use axum::extract::Query;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use crate::models::{Card, Transaction};
use crate::main::AppState;

#[derive(Deserialize)]
pub struct CardInfoQuery {
    pub card_uid: String,
}

#[derive(Deserialize)]
pub struct TopupRequest {
    pub card_uid: String,
    pub amount: serde_json::Decimal,
}

pub async fn get_card_info(
    State(state): State<AppState>,
    Query(query): Query<CardInfoQuery>,
) -> Result<Json<Card>,_> {
    let card = sqlx::query_as::<_, Card>("SELECT * FROM cards WHERE card_uid = $1")
        .bind(query.card_uid)
        .fetch_optional(state.db_pool.clone())
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    match card {
        Some(card) => Ok(Json(card)),
        None => Err(axum::http::StatusCode::NOT_FOUND.into()),
    }
}

pub async fn post_topup(
    State(state): State<AppState>,
    Json(payload): Json<TopupRequest>,
) -> Json<serde_json::_Result<String>> {
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
