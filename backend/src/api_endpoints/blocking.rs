use axum::{
  routing::post,
  Router,
};

pub fn blocking_routes() -> Router {
  Router::new()
    .route("/create", post(blocking_create))
    .route("/delete", post(blocking_delete))
    .route("/list", post(blocking_list))
}

async fn blocking_create() -> &'static str {
  "blocking_create"
}

async fn blocking_delete() -> &'static str {
  "blocking_delete"
}

async fn blocking_list() -> &'static str {
  "blocking_list"
}
