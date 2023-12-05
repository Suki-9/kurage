use axum::{
  routing::post,
  Router,
};

pub fn users_routes() -> Router {
  Router::new()
    .route("/", post(users))
    .route("/clips", post(users_clips))
    .route("/followers", post(users_followers))
    .route("/following", post(users_following))
    .nest("/gallery", Router::new()
      .route("/posts", post(users_gallery_posts))
    )
    .route("/get-frequently-replied-users", post(users_get_frequently_replied_users))
    .nest("/groups", Router::new()
      .route("/create", post(users_groups_create))
      .route("/delete", post(users_groups_delete))
      .nest("/invitations", Router::new()
        .route("/accept", post(users_groups_invitations_accept))
        .route("/rejec", post(users_groups_invitations_rejec))
      )
      .route("/invite", post(users_groups_invite))
      .route("/joined", post(users_groups_joined))
      .route("/owned", post(users_groups_owned))
      .route("/pull", post(users_groups_pull))
      .route("/show", post(users_groups_show))
      .route("/transfer", post(users_groups_transfer))
      .route("/update", post(users_groups_update))
    )
    .nest("/lists", Router::new()
      .route("/create", post(users_lists_create))
      .route("/delete", post(users_lists_delete))
      .route("/list", post(users_lists_list))
      .route("/pull", post(users_lists_pull))
      .route("/push", post(users_lists_push))
      .route("/show", post(users_lists_show))
      .route("/update", post(users_lists_update))
    )
    .route("/notes", post(users_notes))
    .route("/pages", post(users_pages))
    .route("/flashs", post(users_flashs))
    .route("/recommendation", post(users_recommendation))
    .route("/relation", post(users_relation))
    .route("/report-abuse", post(users_report_abuse))
    .route("/search-by-username-and-host", post(users_search_by_username_and_host))
    .route("/search", post(users_search))
    .route("/show", post(users_show))
}

async fn users() -> &'static str {
  "users"
}

async fn users_clips() -> &'static str {
  "users_clips"
}

async fn users_followers() -> &'static str {
  "users_followers"
}

async fn users_following() -> &'static str {
  "users_following"
}

async fn users_gallery_posts() -> &'static str {
  "users_gallery_posts"
}

async fn users_get_frequently_replied_users() -> &'static str {
  "users_get_frequently_replied_users"
}

async fn users_groups_create() -> &'static str {
  "users_groups_create"
}

async fn users_groups_delete() -> &'static str {
  "users_groups_delete"
}

async fn users_groups_invitations_accept() -> &'static str {
  "users_groups_invitations_accept"
}

async fn users_groups_invitations_rejec() -> &'static str {
  "users_groups_invitations_rejec"
}

async fn users_groups_invite() -> &'static str {
  "users_groups_invite"
}

async fn users_groups_joined() -> &'static str {
  "users_groups_joined"
}

async fn users_groups_owned() -> &'static str {
  "users_groups_owned"
}

async fn users_groups_pull() -> &'static str {
  "users_groups_pull"
}

async fn users_groups_show() -> &'static str {
  "users_groups_show"
}

async fn users_groups_transfer() -> &'static str {
  "users_groups_transfer"
}

async fn users_groups_update() -> &'static str {
  "users_groups_update"
}

async fn users_lists_create() -> &'static str {
  "users_lists_create"
}

async fn users_lists_delete() -> &'static str {
  "users_lists_delete"
}

async fn users_lists_list() -> &'static str {
  "users_lists_list"
}

async fn users_lists_pull() -> &'static str {
  "users_lists_pull"
}

async fn users_lists_push() -> &'static str {
  "users_lists_push"
}

async fn users_lists_show() -> &'static str {
  "users_lists_show"
}

async fn users_lists_update() -> &'static str {
  "users_lists_update"
}

async fn users_notes() -> &'static str {
  "users_notes"
}

async fn users_pages() -> &'static str {
  "users_pages"
}

async fn users_flashs() -> &'static str {
  "users_flashs"
}

async fn users_recommendation() -> &'static str {
  "users_recommendation"
}

async fn users_relation() -> &'static str {
  "users_relation"
}

async fn users_report_abuse() -> &'static str {
  "users_report_abuse"
}

async fn users_search_by_username_and_host() -> &'static str {
  "users_search_by_username_and_host"
}

async fn users_search() -> &'static str {
  "users_search"
}

async fn users_show() -> &'static str {
  "users_show"
}