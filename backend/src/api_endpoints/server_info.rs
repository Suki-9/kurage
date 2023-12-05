use axum::{
  routing::post,
  Router,
};

pub fn server_info_routes() -> Router {
  Router::new()
    .route("/", post(server_info))
}

async fn server_info() -> &'static str {
  "server_info"
}