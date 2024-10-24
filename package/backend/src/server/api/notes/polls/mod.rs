use axum::{routing::post, Router};

mod recommendation;
mod vote;

pub(super) fn router() -> Router {
  Router::new()
    .route("/recommendation", post(recommendation::handler))
    .route("/vote", post(vote::handler))
}
