use axum::{routing::post, Router};

mod accept;
mod session;

pub fn router() -> Router {
  Router::new().route("/accept", post(accept::handler)).nest("/session", session::router())
}
