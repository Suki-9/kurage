use axum::{routing::post, Router};

mod drive;
mod following;
mod notes;
mod pv;
mod reactions;

pub(super) fn router() -> Router {
  Router::new()
    .route("/drive", post(drive::handler))
    .route("/following", post(following::handler))
    .route("/notes", post(notes::handler))
    .route("/pv", post(pv::handler))
    .route("/reactions", post(reactions::handler))
}
