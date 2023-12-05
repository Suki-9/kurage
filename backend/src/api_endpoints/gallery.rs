use axum::{
  routing::post,
  Router,
};

pub fn gallery_routes() -> Router {
  Router::new()
    .route("/featured", post(gallery_featured))
    .route("/popular", post(gallery_popular))
    .nest("/post", Router::new()
      .route("/", post(gallery_post))
      .route("/create", post(gallery_post_create))
      .route("/delete", post(gallery_post_delete))
      .route("/like", post(gallery_post_like))
      .route("/show", post(gallery_post_show))
      .route("/unlike", post(gallery_post_unlike))
      .route("/update", post(gallery_post_update))
    )
}

async fn gallery_featured() -> &'static str {
  "gallery_featured"
}

async fn gallery_popular() -> &'static str {
  "gallery_popular"
}

async fn gallery_post() -> &'static str {
  "gallery_post"
}

async fn gallery_post_create() -> &'static str {
  "gallery_post_create"
}

async fn gallery_post_delete() -> &'static str {
  "gallery_post_delete"
}

async fn gallery_post_like() -> &'static str {
  "gallery_post_like"
}

async fn gallery_post_show() -> &'static str {
  "gallery_post_show"
}

async fn gallery_post_unlike() -> &'static str {
  "gallery_post_unlike"
}

async fn gallery_post_update() -> &'static str {
  "gallery_post_update"
}
