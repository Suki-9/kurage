use axum::{routing::post, Router};

mod list;
mod search;
mod show;
mod trend;
mod users;

pub fn router() -> Router {
  Router::new()
    .route("/list", post(list::handler))
    .route("/search", post(search::handler))
    .route("/show", post(show::handler))
    .route("/trend", post(trend::handler))
    .route("/users", post(users::handler))
}
