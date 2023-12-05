use axum::{
  routing::post,
  Router,
};

pub fn reset_password_routes() -> Router {
  Router::new()
    .route("/", post(reset_password))
}

async fn reset_password() -> &'static str {
  "reset_password"
}