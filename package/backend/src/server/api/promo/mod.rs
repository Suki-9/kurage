use axum::{routing::post, Router};

mod read;

pub fn router() -> Router {
  Router::new().route("/read", post(read::handler))
}
