use axum::{routing::post, Router};

mod root;
mod show;

pub fn router() -> Router {
  Router::new().route("/show", post(show::handler)).route("/root", post(root::handler))
}
