use axum::{http::HeaderValue, routing::get, Router};
use hyper::header::CONTENT_TYPE;
use tower_http::cors::{Any, CorsLayer};

pub async fn lunch_server(port: i32) -> () {
  axum::serve(
    tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap(),
    Router::new()
      .route("/", get(|| async { "frontend" })).layer(
      CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers(vec![CONTENT_TYPE]),
    ),
  )
  .await
  .unwrap();
}
