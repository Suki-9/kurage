use axum::{routing::post, Router};

mod get;
mod get_all;
mod get_detail;
mod keys;
mod keys_with_type;
mod remove;
mod scopes_with_domain;
mod set;

pub(super) fn router() -> Router {
  Router::new()
    .route("/get-all", post(get_all::handler))
    .route("/get-detail", post(get_detail::handler))
    .route("/get", post(get::handler))
    .route("/keys-with-type", post(keys_with_type::handler))
    .route("/keys", post(keys::handler))
    .route("/remove", post(remove::handler))
    .route("/scopes-with-domain", post(scopes_with_domain::handler))
    .route("/set", post(set::handler))
}
