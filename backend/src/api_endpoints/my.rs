use axum::{
  routing::post,
  Router,
};

pub fn my_routes() -> Router {
  Router::new()
    .route("/apps", post(my_apps))
}

async fn my_apps() -> &'static str {
  "my_apps"
}