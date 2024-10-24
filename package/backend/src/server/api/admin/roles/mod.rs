use axum::{routing::post, Router};

mod assign;
mod create;
mod delete;
mod list;
mod show;
mod unassign;
mod update;
mod update_default_policies;
mod users;

pub fn router() -> Router {
  Router::new()
    .route("/assign", post(assign::handler))
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/list", post(list::handler))
    .route("/show", post(show::handler))
    .route("/unassign", post(unassign::handler))
    .route("/update-default-policies", post(update_default_policies::handler))
    .route("/update", post(update::handler))
    .route("/users", post(users::handler))
}
