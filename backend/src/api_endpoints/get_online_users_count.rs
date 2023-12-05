use axum::{
  routing::post,
  Router,
};

pub fn get_online_users_coun_routes() -> Router {
  Router::new()
    .route("/", post(get_online_users_count))
}

async fn get_online_users_count() -> &'static str {
  "get_online_users_coun"
}