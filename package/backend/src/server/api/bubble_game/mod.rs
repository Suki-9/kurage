use axum::{routing::post, Router};

mod ranking;
mod register;

pub fn router() -> Router {
  Router::new()
    .route("/ranking", post(ranking::handler))
    .route("/register", post(register::handler))
}
