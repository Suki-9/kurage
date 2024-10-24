use axum::{routing::post, Router};

mod accept;
mod cancel;
mod list;
mod reject;

pub(super) fn router() -> Router {
  Router::new()
    .route("/accept", post(accept::handler))
    .route("/cancel", post(cancel::handler))
    .route("/list", post(list::handler))
    .route("/reject", post(reject::handler))
}
