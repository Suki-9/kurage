use axum::{
  routing::post,
  Router,
};

pub fn drive_routes() -> Router {
  Router::new()
    .route("/", post(drive))
    .nest("/files", Router::new()
      .route("/", post(drive_files))
      .route("/attached-notes", post(drive_files_attached_notes))
      .route("/check-existence", post(drive_files_check_existence))
      .route("/create", post(drive_files_create))
      .route("/delete", post(drive_files_delete))
      .route("/find-by-hash", post(drive_files_find_by_hash))
      .route("/find", post(drive_files_find))
      .route("/show", post(drive_files_show))
      .route("/update", post(drive_files_update))
      .route("/upload-from-url", post(drive_files_upload_from_url))
    )
    .nest("/folders", Router::new()
      .route("/", post(drive_folders))
      .route("/create", post(drive_folders_create))
      .route("/delete", post(drive_folders_delete))
      .route("/find", post(drive_folders_find))
      .route("/show", post(drive_folders_show))
      .route("/update", post(drive_folders_update))
    )
    .route("/stream", post(drive_stream))
}

async fn drive() -> &'static str {
  "drive"
}

async fn drive_files() -> &'static str {
  "drive_files"
}

async fn drive_files_attached_notes() -> &'static str {
  "drive_files_attached_notes"
}

async fn drive_files_check_existence() -> &'static str {
  "drive_files_check_existence"
}

async fn drive_files_create() -> &'static str {
  "drive_files_create"
}

async fn drive_files_delete() -> &'static str {
  "drive_files_delete"
}

async fn drive_files_find_by_hash() -> &'static str {
  "drive_files_find_by_hash"
}

async fn drive_files_find() -> &'static str {
  "drive_files_find"
}

async fn drive_files_show() -> &'static str {
  "drive_files_show"
}

async fn drive_files_update() -> &'static str {
  "drive_files_update"
}

async fn drive_files_upload_from_url() -> &'static str {
  "drive_files_upload_from_url"
}

async fn drive_folders() -> &'static str {
  "drive_folders"
}

async fn drive_folders_create() -> &'static str {
  "drive_folders_create"
}

async fn drive_folders_delete() -> &'static str {
  "drive_folders_delete"
}

async fn drive_folders_find() -> &'static str {
  "drive_folders_find"
}

async fn drive_folders_show() -> &'static str {
  "drive_folders_show"
}

async fn drive_folders_update() -> &'static str {
  "drive_folders_update"
}

async fn drive_stream() -> &'static str {
  "drive_stream"
}
