use axum::{
  routing::post,
  Router,
  response::{
    IntoResponse,
    Json,
  }
};

use serde_json::json;

pub fn endpoint_routes() -> Router {
  Router::new()
    .route("/endpoint", post(endpoint))
    .route("/endpoints", post(endpoint))
}

async fn endpoint() -> impl IntoResponse {
  Json(json!({
    "error": "Unsupported endpoint",
    "summary": "Kurageではこのエンドポイントの実装予定はありません。\n理由: 使いどころが無い為"
  }))
}
