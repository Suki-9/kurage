use axum::{routing::post, Router};

mod create;
mod delete;
mod featured;
mod like;
mod my;
mod my_likes;
mod show;
mod unlike;
mod update;

pub fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/featured", post(featured::handler))
    .route("/like", post(like::handler))
    .route("/my-likes", post(my_likes::handler))
    .route("/my", post(my::handler))
    .route("/show", post(show::handler))
    .route("/unlike", post(unlike::handler))
    .route("/update", post(update::handler))
}
