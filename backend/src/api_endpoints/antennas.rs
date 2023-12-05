use axum::{
  routing::post,
  Router,
};

pub fn antennas_routes() -> Router {
  Router::new()
    .route("/create", post(antennas_create))
    .route("/delete", post(antennas_delete))
    .route("/list", post(antennas_list))
    .route("/notes", post(antennas_notes))
    .route("/show", post(antennas_show))
    .route("/update", post(antennas_update))
}

async fn antennas_create() -> &'static str {
  "antennas_create"
}

async fn antennas_delete() -> &'static str {
  "antennas_delete"
}

async fn antennas_list() -> &'static str {
  "antennas_list"
}

async fn antennas_notes() -> &'static str {
  "antennas_notes"
}

async fn antennas_show() -> &'static str {
  "antennas_show"
}

async fn antennas_update() -> &'static str {
  "antennas_update"
}
