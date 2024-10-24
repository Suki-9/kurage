use axum::{routing::post, Router};

mod posts;

pub fn router() -> Router {
  Router::new().route("/posts", post(posts::handler))
}
