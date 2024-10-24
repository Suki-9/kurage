use axum::Router;

mod notification_recipient;

pub fn router() -> Router {
  Router::new().nest("/notification-recipient", notification_recipient::router())
}
