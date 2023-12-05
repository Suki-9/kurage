use axum::{
  routing::post,
  Router,
};

pub fn promo_routes() -> Router {
  Router::new()
    .route("/read", post(promo_read))
}

async fn promo_read() -> &'static str {
  "promo_read"
}