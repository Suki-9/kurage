use axum::{routing::post, Router};

mod create;

pub(super) fn router() -> Router {
  Router::new().route("/create", post(create::handler))
}
