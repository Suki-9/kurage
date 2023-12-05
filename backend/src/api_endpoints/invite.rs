use axum::{
  routing::post,
  Router,
};

pub fn invite_routes() -> Router {
  Router::new()
    .route("/create", post(invite_create))
    .route("/delete", post(invite_delete))
    .route("/list", post(invite_list))
    .route("/limit", post(invite_limit))
}

async fn invite_create() -> &'static str {
  "invite_create"
}

async fn invite_delete() -> &'static str {
  "invite_delete"
}

async fn invite_list() -> &'static str {
  "invite_list"
}

async fn invite_limit() -> &'static str {
  "invite_limit"
}
