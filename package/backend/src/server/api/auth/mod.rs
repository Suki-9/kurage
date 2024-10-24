use axum::{routing::post, Router};

mod accept;
mod session;

pub(super) fn router() -> Router {
  Router::new().route("/accept", post(accept::handler)).nest("/session", session::router())
}
