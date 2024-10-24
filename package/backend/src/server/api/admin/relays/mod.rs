use axum::{routing::post, Router};

mod add;
mod list;
mod remove;

pub(super) fn router() -> Router {
  Router::new()
    .route("/add", post(add::handler))
    .route("/list", post(list::handler))
    .route("/remove", post(remove::handler))
}
