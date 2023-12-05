use axum::{
  routing::post,
  Router,
};

pub fn messaging_routes() -> Router {
  Router::new()
    .route("/history", post(history))
    .nest("/messages", Router::new()
      .route("/", post(messages))
      .route("/create", post(messages_create))
      .route("/delete", post(messages_delete))
      .route("/read", post(messages_read))
    )
}

async fn history() -> &'static str {
  "history"
}

async fn messages()  -> &'static str {
  "messages"
}

async fn messages_create()  -> &'static str {
  "messages_create"
}

async fn messages_delete()  -> &'static str {
  "messages_delete"
}

async fn messages_read()  -> &'static str {
  "messages_read"
}
