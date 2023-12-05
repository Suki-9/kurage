use axum::{
  routing::post,
  Router,
};

pub fn sw_routes() -> Router {
  Router::new()
    .route("/register", post(sw_register))
}

async fn sw_register() -> &'static str {
  "sw_register"
}