use axum::{
  routing::post,
  Router,
};

pub fn mute_routes() -> Router {
  Router::new()
    .route("/create", post(mute_create))
    .route("/delete", post(mute_delete))
    .route("/list", post(mute_list))
}
  
async fn mute_create() -> &'static str {
  "mute_create"
}

async fn mute_delete() -> &'static str {
  "mute_delete"
}

async fn mute_list() -> &'static str {
  "mute_list"
}