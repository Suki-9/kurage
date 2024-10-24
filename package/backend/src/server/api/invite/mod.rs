use axum::{routing::post, Router};

mod create;
mod delete;
mod limit;
mod list;

pub fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/limit", post(limit::handler))
    .route("/list", post(list::handler))
}
