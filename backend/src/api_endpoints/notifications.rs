use axum::{
  routing::post,
  Router,
};

pub fn notifications_routes() -> Router {
  Router::new()
    .route("/create", post(notifications_create))
    .route("/test-notification", post(notifications_test_notification))
    .route("/mark-all-as-read", post(notifications_mark_all_as_read))
}

async fn notifications_create() -> &'static str {
  "notifications_create"
}

async fn notifications_test_notification() -> &'static str {
  "notifications_test_notification"
}

async fn notifications_mark_all_as_read() -> &'static str {
  "notifications_mark_all_as_read"
}
