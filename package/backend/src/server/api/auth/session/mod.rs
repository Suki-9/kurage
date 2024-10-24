use axum::{routing::post, Router};

mod generate;
mod show;
mod userkey;

pub fn router() -> Router {
  Router::new()
    .route("/generate", post(generate::handler))
    .route("/show", post(show::handler))
    .route("/userkey", post(userkey::handler))
}
