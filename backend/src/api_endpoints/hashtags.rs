use axum::{
  routing::post,
  Router,
};

pub fn hashtags_routes() -> Router {
  Router::new()
    .route("/list", post(hashtags_list))
    .route("/search", post(hashtags_search))
    .route("/show", post(hashtags_show))
    .route("/trend", post(hashtags_trend))
    .route("/users", post(hashtags_users))
}

async fn hashtags_list() -> &'static str {
  "hashtags_list"
}

async fn hashtags_search() -> &'static str {
  "hashtags_search"
}

async fn hashtags_show() -> &'static str {
  "hashtags_show"
}

async fn hashtags_trend() -> &'static str {
  "hashtags_trend"
}

async fn hashtags_users() -> &'static str {
  "hashtags_users"
}
