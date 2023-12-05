use axum::{
  routing::post,
  Router,
};

pub fn notes_routes() -> Router {
  Router::new()
    .route("/", post(notes))
    .route("/children", post(children_note))
    .route("/clips", post(clips))
    .route("/conversation", post(conversation))
    .route("/create", post(create_note))
    .route("/delete", post(delete_note))
    .nest("/favorites", Router::new()
      .route("/create", post(favorites_create))
      .route("/delete", post(favorites_delete))
    )
    .route("/featured", post(featured))
    .route("/global-timeline",post(global_timeline))
    .route("/hybrid-timeline",post(hybrid_timeline))
    .route("/local-timeline", post(local_timeline))
    .route("/mentions", post( mentions))
    .nest("/polls", Router::new()
      .route("/recommendation", post(polls_recommendation))
      .route("/vote", post(polls_vote))
    )
    .nest("/reactions", Router::new()
      .route("/", post(reactions))
      .route("/create", post(reactions_create))
      .route("/delete", post(reactions_delete))
    )
    .route("/renotes", post(renotes))
    .route("/replies", post(replies))
    .route("/search-by-tag", post(search_by_tag))
    .route("/search", post(search))
    .route("/show", post(show))
    .route("/state", post(state))
    .route("/timeline", post(timeline))
    .route("/unrenote", post(unrenote))
    .route("/user-list-timeline", post(user_list_timeline))
    .nest("/watching", Router::new()
      .route("/create", post(watching_create))
      .route("/delete", post(watching_delete))
    )
}

async fn notes() -> &'static str {
  "notes"
}

async fn create_note() -> &'static str {
  "create_note"
}

async fn delete_note() -> &'static str {
  "delete_note"
}

async fn children_note()-> &'static str {
  "children_note"
}

async fn clips() -> &'static str {
  "clips"
}

async fn conversation() -> &'static str {
  "clips"
}

async fn favorites_create() -> &'static str {
  "favorites_create"
}

async fn favorites_delete() -> &'static str {
  "favorites_delete"
}

async fn featured() -> &'static str {
  "featured"
}

async fn global_timeline() -> &'static str {
  "global_timeline"
}

async fn hybrid_timeline() -> &'static str {
  "hybrid_timeline"
}

async fn local_timeline() -> &'static str {
  "local_timeline"
}

async fn polls_vote() -> &'static str {
  "polls_vote"
}

async fn polls_recommendation() -> &'static str {
  "polls_recommendation"
}

async fn mentions() -> &'static str {
  "mentions()"
}

async fn reactions() -> &'static str {
  "reactions"
}

async fn reactions_create() -> &'static str {
  "reactions_create"
}

async fn reactions_delete() -> &'static str {
  "reactions_delete"
}

async fn renotes() -> &'static str {
  "renotes"
}

async fn replies() -> &'static str {
  "replies"
}

async fn search_by_tag() -> &'static str {
  "search_by_tag"
}

async fn search() -> &'static str {
  "search"
}

async fn show() -> &'static str {
  "show"
}

async fn state() -> &'static str {
  "state"
}

async fn timeline() -> &'static str {
  "timeline"
}

async fn unrenote() -> &'static str {
  "unrenote"
}

async fn user_list_timeline() -> &'static str {
  "user_list_timeline"
}

async fn watching_create() -> &'static str {
  "watching_create"
}

async fn watching_delete() -> &'static str {
  "watching_delete"
}
