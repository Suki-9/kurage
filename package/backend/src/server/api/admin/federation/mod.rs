use axum::{routing::post, Router};

mod delete_all_files;
mod refresh_remote_instance_metadata;
mod remove_all_following;
mod update_instance;

pub(super) fn router() -> Router {
  Router::new()
    .route("/delete-all-files", post(delete_all_files::handler))
    .route("/refresh-remote-instance-metadata", post(refresh_remote_instance_metadata::handler))
    .route("/remove-all-following", post(remove_all_following::handler))
    .route("/update-instance", post(update_instance::handler))
}
