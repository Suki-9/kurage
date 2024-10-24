use axum::{routing::post, Router};

mod create;

pub fn router() -> Router {
  Router::new().route("/create", post(create::handler))
}
