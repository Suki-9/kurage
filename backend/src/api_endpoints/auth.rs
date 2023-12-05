use axum::{
  routing::post,
  Router,
};

pub fn auth_routes() -> Router {
  Router::new()
    .route("/accept", post(auth_accept))
    .nest("/session", Router::new()
      .route("/generate", post(auth_session_generate))
      .route("/show", post(auth_session_show))
      .route("/userkey", post(auth_session_userkey))
    )
}

async fn auth_accept() -> &'static str {
  "auth_accept"
}

async fn auth_session_generate() -> &'static str {
  "auth_session_generate"
}

async fn auth_session_show() -> &'static str {
  "auth_session_show"
}

async fn auth_session_userkey() -> &'static str {
  "auth_session_userkey"
}
