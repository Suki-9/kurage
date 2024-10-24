use axum::{routing::post, Router};

mod root;
mod show;

pub(super) fn router() -> Router {
  Router::new().route("/show", post(show::handler)).route("/", post(root::handler))
}
