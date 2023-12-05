use axum::{
  routing::post,
  Router,
};

pub fn fetching_external_data_routes() -> Router {
  Router::new()
    .route("/fetch-rss", post(fetch_rss))
    .route("/fetch-external-resources", post(fetch_external_resources))
}

async fn fetch_rss() -> &'static str {
  "fetch_rss"
}

async fn fetch_external_resources() -> &'static str {
  "fetch_external_resources"
}
