use axum::{
  routing::post,
  Router,
};

pub fn clips_routes() -> Router {
  Router::new()
    .route("/add-note", post(clips_add_note))
    .route("/create", post(clips_create))
    .route("/delete", post(clips_delete))
    .route("/list", post(clips_list))
    .route("/notes", post(clips_notes))
    .route("/show", post(clips_show))
    .route("/update", post(clips_update))
}

async fn clips_add_note() -> &'static str {
  "clips_add_note"
}

async fn clips_create() -> &'static str {
  "clips_create"
}

async fn clips_delete() -> &'static str {
  "clips_delete"
}

async fn clips_list() -> &'static str {
  "clips_list"
}

async fn clips_notes() -> &'static str {
  "clips_notes"
}

async fn clips_show() -> &'static str {
  "clips_show"
}

async fn clips_update() -> &'static str {
  "clips_update"
}
