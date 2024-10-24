use super::api;
use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

pub async fn lunch_server(port: i32) -> () {
  axum::serve(
    tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap(),
    Router::new()
      .nest("/api", api::router())
      .route("/", get(|| async { "frontend" }))
      .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)),
  )
  .await
  .unwrap();
}
