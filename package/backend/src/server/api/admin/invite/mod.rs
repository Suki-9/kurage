use axum::{routing::post, Router};

mod create;
mod list;

pub(super) fn router() -> Router {
  Router::new().route("/create", post(create::handler)).route("/list", post(list::handler))
}
