use axum::{routing::post, Router};

mod get;
mod show;

pub fn router() -> Router {
  Router::new().route("/get", post(get::handler)).route("/show", post(show::handler))
}
