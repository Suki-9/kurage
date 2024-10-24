use axum::{routing::post, Router};

mod create;
mod delete;
mod invalidate;
mod requests;
mod update;
mod update_all;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/invalidate", post(invalidate::handler))
    .route("/update-all", post(update_all::handler))
    .route("/update", post(update::handler))
    .nest("/requests", requests::router())
}
