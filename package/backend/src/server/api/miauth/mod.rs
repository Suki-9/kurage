use axum::{routing::post, Router};

mod gen_token;

pub fn router() -> Router {
  Router::new().route("/gen-token", post(gen_token::handler))
}
