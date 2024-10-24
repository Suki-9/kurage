use axum::{routing::post, Router};

mod list;
mod notes;
mod show;
mod users;

pub fn router() -> Router {
  Router::new()
    .route("/list", post(list::handler))
    .route("/notes", post(notes::handler))
    .route("/show", post(show::handler))
    .route("/users", post(users::handler))
}
