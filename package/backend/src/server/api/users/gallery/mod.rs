use axum::{routing::post, Router};

mod posts;

pub(super) fn router() -> Router {
  Router::new().route("/posts", post(posts::handler))
}
