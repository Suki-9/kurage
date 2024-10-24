use axum::{routing::post, Router};

mod create;
mod flush;
mod mark_all_as_read;
mod test_notification;

pub fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/flush", post(flush::handler))
    .route("/mark-all-as-read", post(mark_all_as_read::handler))
    .route("/test-notification", post(test_notification::handler))
}
