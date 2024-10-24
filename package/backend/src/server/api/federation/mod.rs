use axum::{routing::post, Router};

mod followers;
mod following;
mod instances;
mod show_instance;
mod stats;
mod update_remote_user;
mod users;

pub fn router() -> Router {
  Router::new()
    .route("/followers", post(followers::handler))
    .route("/following", post(following::handler))
    .route("/instances", post(instances::handler))
    .route("/show-instance", post(show_instance::handler))
    .route("/stats", post(stats::handler))
    .route("/update-remote-user", post(update_remote_user::handler))
    .route("/users", post(users::handler))
}
