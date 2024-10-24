use axum::{routing::post, Router};

mod create;
mod delete;
mod root;

pub fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/root", post(root::handler))
}
