use axum::{routing::post, Router};

mod create;
mod delete;
mod list;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/list", post(list::handler))
}
