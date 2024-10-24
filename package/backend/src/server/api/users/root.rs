use axum::{
  http::StatusCode,
  response::{IntoResponse, Json},
};
use serde_json::json;

pub(super) async fn handler() -> impl IntoResponse {
  (
    StatusCode::NOT_IMPLEMENTED,
    Json(json!({
      "error": "This api is not yet implemented.",
    })),
  )
}
