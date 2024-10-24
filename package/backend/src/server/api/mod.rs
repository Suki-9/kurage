use axum::{routing::post, Router};

mod admin;
mod announcements;
mod antennas;
mod ap;
mod app;
mod auth;
mod blocking;
mod bubble_game;
mod channels;
mod charts;
mod clips;
mod drive;
mod email_address;
mod emoji;
mod emojis;
mod endpoint;
mod endpoints;
mod export_custom_emojis;
mod federation;
mod fetch_external_resources;
mod fetch_rss;
mod flash;
mod following;
mod gallery;
mod get_avatar_decorations;
mod get_online_users_count;
mod hashtags;
mod i;
mod invite;
mod meta;
mod miauth;
mod mute;
mod my;
mod notes;
mod notifications;
mod page_push;
mod pages;
mod ping;
mod pinned_users;
mod promo;
mod renote_mute;
mod request_reset_password;
mod reset_db;
mod reset_password;
mod retention;
mod reversi;
mod roles;
mod server_info;
mod stats;
mod sw;
mod test;
mod username;
mod users;

pub(super) fn router() -> Router {
  Router::new()
    .route("/emoji", post(emoji::handler))
    .route("/emojis", post(emojis::handler))
    .route("/endpoint", post(endpoint::handler))
    .route("/endpoints", post(endpoints::handler))
    .route("/export-custom-emojis", post(export_custom_emojis::handler))
    .route("/fetch-external-resources", post(fetch_external_resources::handler))
    .route("/fetch-rss", post(fetch_rss::handler))
    .route("/get-avatar-decorations", post(get_avatar_decorations::handler))
    .route("/get-online-users-count", post(get_online_users_count::handler))
    .route("/meta", post(meta::handler))
    .route("/page-push", post(page_push::handler))
    .route("/ping", post(ping::handler))
    .route("/pinned-users", post(pinned_users::handler))
    .route("/request-reset-password", post(request_reset_password::handler))
    .route("/reset-db", post(reset_db::handler))
    .route("/reset-password", post(reset_password::handler))
    .route("/retention", post(retention::handler))
    .route("/server-info", post(server_info::handler))
    .route("/stats", post(stats::handler))
    .route("/test", post(test::handler))
    .nest("/admin", admin::router())
    .nest("/announcements", announcements::router())
    .nest("/antennas", antennas::router())
    .nest("/ap", ap::router())
    .nest("/app", app::router())
    .nest("/auth", auth::router())
    .nest("/blocking", blocking::router())
    .nest("/bubble-game", bubble_game::router())
    .nest("/channels", channels::router())
    .nest("/charts", charts::router())
    .nest("/clips", clips::router())
    .nest("/drive", drive::router())
    .nest("/email-address", email_address::router())
    .nest("/federation", federation::router())
    .nest("/flash", flash::router())
    .nest("/following", following::router())
    .nest("/gallery", gallery::router())
    .nest("/hashtags", hashtags::router())
    .nest("/i", i::router())
    .nest("/invite", invite::router())
    .nest("/miauth", miauth::router())
    .nest("/mute", mute::router())
    .nest("/my", my::router())
    .nest("/notes", notes::router())
    .nest("/notifications", notifications::router())
    .nest("/pages", pages::router())
    .nest("/promo", promo::router())
    .nest("/renote-mute", renote_mute::router())
    .nest("/reversi", reversi::router())
    .nest("/roles", roles::router())
    .nest("/sw", sw::router())
    .nest("/username", username::router())
    .nest("/users", users::router())
}
