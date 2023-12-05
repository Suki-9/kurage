use axum::{
  routing::post,
  Router,
};

pub fn stat_routes() -> Router {
  Router::new()
    .route("/", post(stats))
}

async fn stats() -> &'static str {
  "stats"
}