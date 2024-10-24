use axum::{routing::post, Router};

mod children;
mod clips;
mod conversation;
mod create;
mod delete;
mod favorites;
mod featured;
mod global_timeline;
mod hybrid_timeline;
mod local_timeline;
mod mentions;
mod polls;
mod reactions;
mod renotes;
mod replies;
mod root;
mod search;
mod search_by_tag;
mod show;
mod state;
mod thread_muting;
mod timeline;
mod translate;
mod unrenote;
mod user_list_timeline;

pub fn router() -> Router {
  Router::new()
    .route("/children", post(children::handler))
    .route("/clips", post(clips::handler))
    .route("/conversation", post(conversation::handler))
    .route("/create", post(create::handler))
    .route("/delete", post(delete::handler))
    .route("/featured", post(featured::handler))
    .route("/global-timeline", post(global_timeline::handler))
    .route("/hybrid-timeline", post(hybrid_timeline::handler))
    .route("/local-timeline", post(local_timeline::handler))
    .route("/mentions", post(mentions::handler))
    .route("/renotes", post(renotes::handler))
    .route("/replies", post(replies::handler))
    .route("/search-by-tag", post(search_by_tag::handler))
    .route("/search", post(search::handler))
    .route("/show", post(show::handler))
    .route("/state", post(state::handler))
    .route("/timeline", post(timeline::handler))
    .route("/translate", post(translate::handler))
    .route("/unrenote", post(unrenote::handler))
    .route("/user-list-timeline", post(user_list_timeline::handler))
    .route("/root", post(root::handler))
    .nest("/favorites", favorites::router())
    .nest("/polls", polls::router())
    .nest("/reactions", reactions::router())
    .nest("/thread-muting", thread_muting::router())
}
