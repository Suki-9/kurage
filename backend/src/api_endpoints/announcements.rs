use axum::{
  routing::post,
  Router,
};

pub fn announcements_routes() -> Router {
  Router::new()
    .route("/", post(announcements))
}

async fn announcements() -> &'static str {
  "announcements"
}