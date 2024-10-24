use axum::{routing::post, Router};

mod create;
mod delete;
mod find_by_email;

pub fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/find-by-email", post(find_by_email::handler))
}
