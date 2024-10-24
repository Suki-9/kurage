use axum::{routing::post, Router};

mod apps;

pub(super) fn router() -> Router {
  Router::new().route("/apps", post(apps::handler))
}
