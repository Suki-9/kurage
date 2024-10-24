use axum::{routing::post, Router};

mod featured;
mod popular;
mod posts;

pub fn router() -> Router {
  Router::new()
    .route("/featured", post(featured::handler))
    .route("/popular", post(popular::handler))
    .nest("/posts", posts::router())
}
