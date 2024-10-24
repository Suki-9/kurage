use axum::{routing::post, Router};

mod cancel_match;
mod games;
mod invitations;
mod r#match;
mod show_game;
mod surrender;
mod verify;

pub fn router() -> Router {
  Router::new()
    .route("/cancel-match", post(cancel_match::handler))
    .route("/games", post(games::handler))
    .route("/invitations", post(invitations::handler))
    .route("/match", post(r#match::handler))
    .route("/show-game", post(show_game::handler))
    .route("/surrender", post(surrender::handler))
    .route("/verify", post(verify::handler))
}
