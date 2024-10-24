use axum::{routing::post, Router};

mod likes;
mod posts;

pub fn router() -> Router {
  Router::new().route("/likes", post(likes::handler)).route("/posts", post(posts::handler))
}