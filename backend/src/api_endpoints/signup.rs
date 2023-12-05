use axum::{
  routing::post,
  Router,
};

pub fn signup_routes() -> Router {
  Router::new()
    .route("/", post(signup))
}

async fn signup() -> &'static str {
  "signup"
}