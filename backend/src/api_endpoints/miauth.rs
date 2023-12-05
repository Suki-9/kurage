use axum::{
  routing::post,
  Router,
};
  
pub fn miauth_routes() -> Router {
  Router::new()
    .route("/gen-token", post(gen_token))
}

async fn gen_token() -> &'static str {
  "gen_token"
}