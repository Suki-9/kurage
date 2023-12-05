use axum::{
  routing::post,
  Router,
};

pub fn page_push_routes() -> Router {
  Router::new()
    .route("/page-push", post(page_push))
}

async fn page_push() -> &'static str {
  "page_push"
}