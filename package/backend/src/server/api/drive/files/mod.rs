use axum::{routing::post, Router};

mod attached_notes;
mod check_existence;
mod create;
mod delete;
mod find;
mod find_by_hash;
mod root;
mod show;
mod update;
mod upload_from_url;

pub fn router() -> Router {
  Router::new()
    .route("/attached-notes", post(attached_notes::handler))
    .route("/check-existence", post(check_existence::handler))
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/find-by-hash", post(find_by_hash::handler))
    .route("/find", post(find::handler))
    .route("/show", post(show::handler))
    .route("/update", post(update::handler))
    .route("/upload-from-url", post(upload_from_url::handler))
    .route("/root", post(root::handler))
}
