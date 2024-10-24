use axum::{routing::post, Router};

mod create;
mod show;

pub fn router() -> Router {
  Router::new().route("/create", post(create::handler)).route("/show", post(show::handler))
}
