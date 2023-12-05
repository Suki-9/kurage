use axum::{
  routing::post,
  Router,
};

pub fn endpoint_routes() -> Router {
  Router::new()
    .route("/endpoint", post(endpoint))
    .route("/endpoints", post(endpoints))
}

async fn endpoint() -> &'static str {
  "endpoint"
}

async fn endpoints() -> &'static str {
  "endpoints"
}