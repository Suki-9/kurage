use axum::{routing::post, Router};

mod add_note;
mod create;
mod delete;
mod favorite;
mod list;
mod my_favorites;
mod notes;
mod remove_note;
mod show;
mod unfavorite;
mod update;

pub(super) fn router() -> Router {
  Router::new()
    .route("/add-note", post(add_note::handler))
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/favorite", post(favorite::handler))
    .route("/list", post(list::handler))
    .route("/my-favorites", post(my_favorites::handler))
    .route("/notes", post(notes::handler))
    .route("/remove-note", post(remove_note::handler))
    .route("/show", post(show::handler))
    .route("/unfavorite", post(unfavorite::handler))
    .route("/update", post(update::handler))
}
