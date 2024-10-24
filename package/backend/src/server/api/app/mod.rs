use axum::{routing::post, Router};

mod create;
mod show;

pub(super) fn router() -> Router {
  Router::new().route("/create", post(create::handler)).route("/show", post(show::handler))
}
