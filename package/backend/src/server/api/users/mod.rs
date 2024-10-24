use axum::{routing::post, Router};

mod achievements;
mod clips;
mod featured_notes;
mod flashs;
mod followers;
mod following;
mod gallery;
mod get_frequently_replied_users;
mod lists;
mod notes;
mod pages;
mod reactions;
mod recommendation;
mod relation;
mod report_abuse;
mod root;
mod search;
mod search_by_username_and_host;
mod show;
mod update_memo;

pub(super) fn router() -> Router {
  Router::new()
    .route("/achievements", post(achievements::handler))
    .route("/clips", post(clips::handler))
    .route("/featured-notes", post(featured_notes::handler))
    .route("/flashs", post(flashs::handler))
    .route("/followers", post(followers::handler))
    .route("/following", post(following::handler))
    .route("/get-frequently-replied-users", post(get_frequently_replied_users::handler))
    .route("/notes", post(notes::handler))
    .route("/pages", post(pages::handler))
    .route("/reactions", post(reactions::handler))
    .route("/recommendation", post(recommendation::handler))
    .route("/relation", post(relation::handler))
    .route("/report-abuse", post(report_abuse::handler))
    .route("/search-by-username-and-host", post(search_by_username_and_host::handler))
    .route("/search", post(search::handler))
    .route("/show", post(show::handler))
    .route("/update-memo", post(update_memo::handler))
    .route("/", post(root::handler))
    .nest("/gallery", gallery::router())
    .nest("/lists", lists::router())
}
