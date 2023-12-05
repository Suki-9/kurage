use axum::{
  routing::post,
  Router,
};

pub fn federation_routes() -> Router {
  Router::new()
    .route("/dns", post(federation_dns))
    .route("/followers", post(federation_followers))
    .route("/instances", post(federation_instances))
    .route("/show-instance", post(federation_show_instance))
    .route("/update-remote-user", post(federation_update_remote_user))
    .route("/users", post(federation_users))
}

async fn federation_dns() -> &'static str {
  "federation_dns"
}

async fn federation_followers() -> &'static str {
  "federation_followers"
}

async fn federation_instances() -> &'static str {
  "federation_instances"
}

async fn federation_show_instance() -> &'static str {
  "federation_show_instance"
}


async fn federation_update_remote_user() -> &'static str {
  "federation_update_remote_user"
}


async fn federation_users() -> &'static str {
  "federation_users"
}
