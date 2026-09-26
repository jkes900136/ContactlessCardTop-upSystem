use axum::routing::{get, post};
use axum::Router;

pub fn api_routes() -> Router {
    Router::new()
        .route("/card/info", get(get_card_info))
        .route("/topup", post(post_topup))
        .route("/transactions", get(get_transactions))
        .route("/status", get(get_status))
}

// Handlers will be imported from the handlers module
use crate::handlers::{
    get_card_info, get_status, get_transactions, post_topup,
};
