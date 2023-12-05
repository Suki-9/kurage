use axum::{
  routing::post,
  Router,
};

pub fn meta_routes() -> Router {
  Router::new()
    .route("/", post(meta))
}

async fn meta() -> &'static str {
  "meta"
}