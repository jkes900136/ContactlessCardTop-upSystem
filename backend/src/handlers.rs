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
) -> Result<Json<serde_json::_Result<String>>, axum::http::StatusCode> {
    let mut tx = sqlx::query("BEGIN")
        .execute(&state.db_pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // 1. Find and validate card
    let card_exists = sqlx::query("SELECT id FROM cards WHERE card_uid = $1 AND status = 'active'")
        .bind(&payload.card_uid)
        .fetch_optional(&mut *tx) // Note: In production, you'd use a transaction object correctly
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if card_exists.is_none() {
        return Err(axum::http::StatusCode::NOT_FOUND);
    }

    // 2. Update balance
    sqlx::query("UPDATE cards SET balance = balance + $1 WHERE card_uid = $2")
        .bind(payload.amount)
        .bind(&payload.card_uid)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http_status::StatusCode::INTERNAL_SERVER_ERROR)?;

    // 3. Record transaction
    sqlx::query("INSERT INTO transactions (card_uid, amount, timestamp) VALUES ($1, $2, NOW())")
        .bind(&payload.card_uid)
        .bind(payload.amount)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http_status::StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("COMMIT")
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http_status::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Ok(format!("Successfully topped up {} for card {}", payload.amount, payload.card_uid))))
}

pub async fn get_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, axum::http::StatusCode> {
    let transactions = sqlx::query_as::<_, Transaction>("SELECT * FROM transactions")
        .fetch_all(state.db_pool.clone())
        .await
        .map_err(|_e| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(transactions))
}

pub async fn get_status(
    State(state): State<AppState>,
) -> Result<Json<serde_json::_Result<String>>, axum::http::StatusCode> {
    let _ = sqlx::query("SELECT 1").fetch_one(&state.db_pool).await
        .map_err(|_e| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(Ok("System is healthy".to_string())))
}
