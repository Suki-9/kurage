use axum::{
  routing::post,
  Router,
};

pub fn username_routes() -> Router {
  Router::new()
    .route("/available", post(username_available))
}

async fn username_available() -> &'static str {
  "available"
}