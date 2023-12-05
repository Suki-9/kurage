use axum::{
  routing::post,
  Router,
};

pub fn room_routes() -> Router {
  Router::new()
    .route("/show", post(room_show))
    .route("/update", post(room_update))
}

async fn room_show() -> &'static str {
  "room_show"
}

async fn room_update() -> &'static str {
  "room_update"
}