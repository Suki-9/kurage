use axum::{routing::post, Router};

mod create;
mod favorite;
mod featured;
mod follow;
mod followed;
mod my_favorites;
mod owned;
mod search;
mod show;
mod timeline;
mod unfavorite;
mod unfollow;
mod update;

pub(super) fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/favorite", post(favorite::handler))
    .route("/featured", post(featured::handler))
    .route("/follow", post(follow::handler))
    .route("/followed", post(followed::handler))
    .route("/my-favorites", post(my_favorites::handler))
    .route("/owned", post(owned::handler))
    .route("/search", post(search::handler))
    .route("/show", post(show::handler))
    .route("/timeline", post(timeline::handler))
    .route("/unfavorite", post(unfavorite::handler))
    .route("/unfollow", post(unfollow::handler))
    .route("/update", post(update::handler))
}
