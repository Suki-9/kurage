use axum::{routing::post, Router};

mod create;
mod delete;
mod list;
mod notes;
mod show;
mod update;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/list", post(list::handler))
    .route("/notes", post(notes::handler))
    .route("/show", post(show::handler))
    .route("/update", post(update::handler))
}
