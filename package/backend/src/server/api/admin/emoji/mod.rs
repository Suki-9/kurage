use axum::{routing::post, Router};

mod add;
mod add_aliases_bulk;
mod copy;
mod delete;
mod delete_bulk;
mod import_zip;
mod list;
mod list_remote;
mod remove_aliases_bulk;
mod set_aliases_bulk;
mod set_category_bulk;
mod set_license_bulk;
mod update;

pub(super) fn router() -> Router {
  Router::new()
    .route("/add-aliases-bulk", post(add_aliases_bulk::handler))
    .route("/add", post(add::handler))
    .route("/copy", post(copy::handler))
    .route("/delete-bulk", post(delete_bulk::handler))
    .route("/delete", post(delete::handler))
    .route("/import-zip", post(import_zip::handler))
    .route("/list-remote", post(list_remote::handler))
    .route("/list", post(list::handler))
    .route("/remove-aliases-bulk", post(remove_aliases_bulk::handler))
    .route("/set-aliases-bulk", post(set_aliases_bulk::handler))
    .route("/set-category-bulk", post(set_category_bulk::handler))
    .route("/set-license-bulk", post(set_license_bulk::handler))
    .route("/update", post(update::handler))
}
