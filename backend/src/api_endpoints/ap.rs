use axum::{
  routing::post,
  Router,
};

pub fn ap_routes() -> Router {
  Router::new()
    .route("/get", post(ap_get))
    .route("/show", post(ap_show))
}

async fn ap_get() -> &'static str {
  "ap_get"
}

async fn ap_show() -> &'static str {
  "ap_show"
}
