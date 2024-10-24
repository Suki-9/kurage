use axum::{routing::post, Router};

mod create;
mod create_from_public;
mod delete;
mod favorite;
mod get_memberships;
mod list;
mod pull;
mod push;
mod show;
mod unfavorite;
mod update;
mod update_membership;

pub fn router() -> Router {
  Router::new()
    .route("/create-from-public", post(create_from_public::handler))
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/favorite", post(favorite::handler))
    .route("/get-memberships", post(get_memberships::handler))
    .route("/list", post(list::handler))
    .route("/pull", post(pull::handler))
    .route("/push", post(push::handler))
    .route("/show", post(show::handler))
    .route("/unfavorite", post(unfavorite::handler))
    .route("/update-membership", post(update_membership::handler))
    .route("/update", post(update::handler))
}
