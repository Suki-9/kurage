use axum::{
  routing::post,
  Router,
};

pub fn pages_routes() -> Router {
  Router::new()
    .route("/create", post(pages_create))
    .route("/delete", post(pages_delete))
    .route("/featured", post(pages_featured))
    .route("/like", post(pages_like))
    .route("/show", post(pages_show))
    .route("/unlike", post(pages_unlike))
    .route("/update", post(pages_update))
}

async fn pages_create() -> &'static str {
  "pages_create"
}

async fn pages_delete() -> &'static str {
  "pages_create"
}

async fn pages_featured() -> &'static str {
  "pages_create"
}

async fn pages_like() -> &'static str {
  "pages_like"
}

async fn pages_show() -> &'static str {
  "pages_show"
}

async fn pages_unlike() -> &'static str {
  "pages_unlike"
}

async fn pages_update() -> &'static str {
  "pages_update"
}
