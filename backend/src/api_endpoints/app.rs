use axum::{
  routing::post,
  Router,
};

pub fn app_routes() -> Router {
  Router::new()
    .route("/create", post(app_create))
    .route("/show", post(app_show))
}

async fn app_create() -> &'static str {
  "app_create"
}

async fn app_show() -> &'static str {
  "app_show"
}