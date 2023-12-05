use axum::{
  routing::post,
  Router,
};

pub fn request_reset_password_routes() -> Router {
  Router::new()
    .route("/", post(request_reset_password))
}

async fn request_reset_password() -> &'static str {
  "request_reset_password"
}