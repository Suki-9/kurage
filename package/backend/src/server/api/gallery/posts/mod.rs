use axum::{routing::post, Router};

mod create;
mod delete;
mod like;
mod root;
mod show;
mod unlike;
mod update;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/like", post(like::handler))
    .route("/show", post(show::handler))
    .route("/unlike", post(unlike::handler))
    .route("/update", post(update::handler))
    .route("/", post(root::handler))
}
