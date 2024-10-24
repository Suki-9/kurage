use axum::{routing::post, Router};

mod active_users;
mod ap_request;
mod drive;
mod federation;
mod instance;
mod notes;
mod user;
mod users;

pub fn router() -> Router {
  Router::new()
    .route("/active-users", post(active_users::handler))
    .route("/ap-request", post(ap_request::handler))
    .route("/drive", post(drive::handler))
    .route("/federation", post(federation::handler))
    .route("/instance", post(instance::handler))
    .route("/notes", post(notes::handler))
    .route("/users", post(users::handler))
    .nest("/user", user::router())
}
