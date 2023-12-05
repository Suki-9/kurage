use axum::{
  routing::post,
  Router,
};

pub fn pinned_users_routes() -> Router {
  Router::new()
    .route("/", post(pinned_users))
}

async fn pinned_users() -> &'static str {
  "pinned_users"
}