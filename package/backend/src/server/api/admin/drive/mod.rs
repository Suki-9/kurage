use axum::{routing::post, Router};

mod clean_remote_files;
mod cleanup;
mod files;
mod show_file;

pub(super) fn router() -> Router {
  Router::new()
    .route("/clean-remote-files", post(clean_remote_files::handler))
    .route("/cleanup", post(cleanup::handler))
    .route("/files", post(files::handler))
    .route("/show-file", post(show_file::handler))
}
