use axum::{routing::post, Router};

mod create;
mod delete;
mod find;
mod root;
mod show;
mod update;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/find", post(find::handler))
    .route("/show", post(show::handler))
    .route("/update", post(update::handler))
    .route("/", post(root::handler))
}
