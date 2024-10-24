use axum::{routing::post, Router};

mod register;
mod show_registration;
mod unregister;
mod update_registration;

pub(super) fn router() -> Router {
  Router::new()
    .route("/register", post(register::handler))
    .route("/show-registration", post(show_registration::handler))
    .route("/unregister", post(unregister::handler))
    .route("/update-registration", post(update_registration::handler))
}
