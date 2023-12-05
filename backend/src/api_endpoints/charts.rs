use axum::{
  routing::post,
  Router,
};

pub fn charts_routes() -> Router {
  Router::new()
    .route("/active-users", post(charts_active_users))
    .route("/drive", post(charts_drive))
    .route("/federation", post(charts_federation))
    .route("/hashtag", post(charts_hashtag))
    .route("/instance", post(charts_instance))
    .route("/network", post(charts_network))
    .route("/notes", post(charts_notes))
    .nest("/user", Router::new()
      .route("/", post(charts_user))
      .route("/drive", post(charts_user_drive))
      .route("/following", post(charts_user_following))
      .route("/notes", post(charts_user_notes))
      .route("/users", post(charts_user_users))
    )
}

async fn charts_active_users() -> &'static str {
  "charts_active_users"
}

async fn charts_drive() -> &'static str {
  "charts_drive"
}

async fn charts_federation() -> &'static str {
  "charts_federation"
}

async fn charts_hashtag() -> &'static str {
  "charts_hashtag"
}

async fn charts_instance() -> &'static str {
  "charts_instance"
}

async fn charts_network() -> &'static str {
  "charts_network"
}

async fn charts_notes() -> &'static str {
  "charts_notes"
}

async fn charts_user() -> &'static str {
  "charts_user"
}

async fn charts_user_drive() -> &'static str {
  "charts_user_drive"
}

async fn charts_user_following() -> &'static str {
  "charts_user_following"
}

async fn charts_user_notes() -> &'static str {
  "charts_user_notes"
}

async fn charts_user_users() -> &'static str {
  "charts_user_users"
}
