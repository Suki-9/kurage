use axum::{routing::post, Router};

mod files;
mod folders;
mod root;
mod stream;

pub fn router() -> Router {
  Router::new()
    .route("/stream", post(stream::handler))
    .route("/root", post(root::handler))
    .nest("/files", files::router())
    .nest("/folders", folders::router())
}
