use axum::{routing::post, Router};

mod files;
mod folders;
mod root;
mod stream;

pub(super) fn router() -> Router {
  Router::new()
    .route("/stream", post(stream::handler))
    .route("/", post(root::handler))
    .nest("/files", files::router())
    .nest("/folders", folders::router())
}
