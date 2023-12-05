use axum::{
  routing::post,
  Router,
};

pub fn following_routes() -> Router {
  Router::new()
    .route("/create", post(following_create))
    .route("/delete", post(following_delete))
    .nest("/requests", Router::new()
      .route("/accept", post(following_requests_accept))
      .route("/cancel", post(following_requests_cancel))
      .route("/list", post(following_requests_list))
      .route("/reject", post(following_requests_reject))
    )
}

async fn following_create() -> &'static str {
  "following_create"
}

async fn following_delete() -> &'static str {
  "following_delete"
}

async fn following_requests_accept() -> &'static str {
  "following_requests_accept"
}

async fn following_requests_cancel() -> &'static str {
  "following_requests_cancel"
}

async fn following_requests_list() -> &'static str {
  "following_requests_list"
}

async fn following_requests_reject() -> &'static str {
  "following_requests_reject"
}
