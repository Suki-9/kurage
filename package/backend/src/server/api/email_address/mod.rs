use axum::{routing::post, Router};

mod available;

pub(super) fn router() -> Router {
  Router::new().route("/available", post(available::handler))
}
