mod api_endpoints;

use tower_http::cors::{
  CorsLayer,
  Any,
};

use axum::{
  Router,
  routing::get,
};

#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
  url: String,
  port: String,
}

#[tokio::main]
async fn main() {
  let config: Config = serde_yaml::from_str(
    &std::fs::read_to_string("./src/config/default.yaml").unwrap()
  ).unwrap();

  let app = Router::new()
    .nest("/api", Router::new()
      .nest("/admin", api_endpoints::admin::admin_routes())
      .nest("/announcements", api_endpoints::announcements::announcements_routes())
      .nest("/antennas", api_endpoints::antennas::antennas_routes())
      .nest("/ap", api_endpoints::ap::ap_routes())
      .nest("/app", api_endpoints::app::app_routes())
      .nest("/auth", api_endpoints::auth::auth_routes())
      .nest("/blocking", api_endpoints::blocking::blocking_routes())
      .nest("/channels", api_endpoints::channels::channels_routes())
      .nest("/charts", api_endpoints::charts::charts_routes())
      .nest("/clips", api_endpoints::clips::clips_routes())
      .nest("/drive", api_endpoints::drive::drive_routes())
      .nest("/federation", api_endpoints::federation::federation_routes())
      .nest("/following", api_endpoints::following::following_routes())
      .nest("/gallery", api_endpoints::gallery::gallery_routes())
      .nest("/games", api_endpoints::games::games_routes())
      .nest("/get_online_users_coun", api_endpoints::get_online_users_count::get_online_users_coun_routes())
      .nest("/hashtags", api_endpoints::hashtags::hashtags_routes())
      .nest("/i", api_endpoints::i::i_routes())
      .nest("/invite", api_endpoints::invite::invite_routes())
      .nest("/messaging", api_endpoints::messaging::messaging_routes())
      .nest("/meta", api_endpoints::meta::meta_routes())
      .nest("/miauth", api_endpoints::miauth::miauth_routes())
      .nest("/mute", api_endpoints::mute::mute_routes())
      .nest("/my", api_endpoints::my::my_routes())
      .nest("/notes", api_endpoints::notes::notes_routes())
      .nest("/notifications", api_endpoints::notifications::notifications_routes())
      .nest("/page_push", api_endpoints::page_push::page_push_routes())
      .nest("/pages", api_endpoints::pages::pages_routes())
      .nest("/ping", api_endpoints::ping::ping_routes())
      .nest("/pinned-users", api_endpoints::pinned_users::pinned_users_routes())
      .nest("/promo", api_endpoints::promo::promo_routes())
      .nest("/request-reset-password", api_endpoints::request_reset_password::request_reset_password_routes())
      .nest("/reset-password", api_endpoints::reset_password::reset_password_routes())
      .nest("/room", api_endpoints::room::room_routes())
      .nest("/signup", api_endpoints::signup::signup_routes())
      .nest("/stats", api_endpoints::stats::stat_routes())
      .nest("/server-info", api_endpoints::server_info::server_info_routes())
      .nest("/sw", api_endpoints::sw::sw_routes())
      .nest("/username", api_endpoints::username::username_routes())
      .nest("/users", api_endpoints::users::users_routes())
      .nest("/", api_endpoints::fetching_external_data::fetching_external_data_routes())
      .nest("/", api_endpoints::endpoints::endpoint_routes())
    )
    .route("/", get(|| async { "frontend" }))
    .route("/nodeinfo/2.1", get(|| async { "I am Kurage." }))
    .layer(CorsLayer::new()
      .allow_origin(Any)
    );

  let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await.unwrap();

  axum::serve(listener, app).await.unwrap();
}
