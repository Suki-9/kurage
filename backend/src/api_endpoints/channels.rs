use axum::{
  routing::post,
  Router,
};

pub fn channels_routes() -> Router {
  Router::new()
    .route("/create", post(channels_create))
    .route("/featured", post(channels_featured))
    .route("/follow", post(channels_follow))
    .route("/followed", post(channels_followed))
    .route("/owned", post(channels_owned))
    .route("/pin-note", post(channels_pin_note))
    .route("/timeline", post(channels_timeline))
    .route("/unfollow", post(channels_unfollow))
    .route("/update", post(channels_update))
}

async fn channels_create() -> &'static str {
  "channels_create"
}

async fn channels_featured() -> &'static str {
  "channels_featured"
}

async fn channels_follow() -> &'static str {
  "channels_follow"
}

async fn channels_followed() -> &'static str {
  "channels_followed"
}

async fn channels_owned() -> &'static str {
  "channels_owned"
}

async fn channels_pin_note() -> &'static str {
  "channels_pin_note"
}

async fn channels_timeline() -> &'static str {
  "channels_timeline"
}

async fn channels_unfollow() -> &'static str {
  "channels_unfollow"
}

async fn channels_update() -> &'static str {
  "channels_update"
}
