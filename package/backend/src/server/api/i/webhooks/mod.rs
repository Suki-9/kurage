use axum::{routing::post, Router};

mod create;
mod delete;
mod list;
mod show;
mod test;
mod update;

pub fn router() -> Router {
  Router::new()
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/list", post(list::handler))
    .route("/show", post(show::handler))
    .route("/test", post(test::handler))
    .route("/update", post(update::handler))
}
