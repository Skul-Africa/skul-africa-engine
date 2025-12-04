use axum::{
    Router,
    routing::{get, post},
    extract::Json,
    response::IntoResponse,
};
use serde_json::Value;
use crate::router::handle_command;

pub fn create_router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/command", post(run_command))
}

async fn ping() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "message": "pong"
    }))
}

async fn run_command(Json(body): Json<Value>) -> impl IntoResponse {
    match serde_json::to_string(&body)
        .ok()
        .and_then(|s| handle_command(&s).ok())
    {
        Some(v) => Json(v),
        None => Json(serde_json::json!({
            "status": "error",
            "error": "Invalid command"
        })),
    }
}
