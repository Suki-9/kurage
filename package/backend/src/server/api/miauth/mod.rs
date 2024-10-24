use axum::{routing::post, Router};

mod gen_token;

pub(super) fn router() -> Router {
  Router::new().route("/gen-token", post(gen_token::handler))
}
