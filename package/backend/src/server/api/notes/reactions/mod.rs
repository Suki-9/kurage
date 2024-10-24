use axum::{routing::post, Router};

mod create;
mod delete;
mod root;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/", post(root::handler))
}
