use axum::{routing::post, Router};

mod clear;
mod deliver_delayed;
mod inbox_delayed;
mod promote;
mod stats;

pub(super) fn router() -> Router {
  Router::new()
    .route("/clear", post(clear::handler))
    .route("/deliver-delayed", post(deliver_delayed::handler))
    .route("/inbox-delayed", post(inbox_delayed::handler))
    .route("/promote", post(promote::handler))
    .route("/stats", post(stats::handler))
}
