use axum::{routing::post, Router};

mod done;
mod key_done;
mod password_less;
mod register;
mod register_key;
mod remove_key;
mod unregister;
mod update_key;

pub(super) fn router() -> Router {
  Router::new()
    .route("/done", post(done::handler))
    .route("/key-done", post(key_done::handler))
    .route("/password-less", post(password_less::handler))
    .route("/register-key", post(register_key::handler))
    .route("/register", post(register::handler))
    .route("/remove-key", post(remove_key::handler))
    .route("/unregister", post(unregister::handler))
    .route("/update-key", post(update_key::handler))
}
