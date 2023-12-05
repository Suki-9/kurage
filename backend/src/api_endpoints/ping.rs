use axum::{
  routing::post,
  Router,
};

pub fn ping_routes() -> Router {
  Router::new()
    .route("/", post(ping))
}

async fn ping() -> &'static str {
  "ping"
}