use axum::{
  routing::post,
  Router,
};

pub fn games_routes() -> Router {
  Router::new()
  .nest("/reversi", Router::new()
    .nest("/games", Router::new()
      .route("/", post(games_reversi_games))
      .route("/show", post(games_reversi_games_show))
      .route("/surrender", post(games_reversi_games_surrender))
    )
    .route("/invitations", post(games_reversi_invitations))
    .nest("/match", Router::new()
      .route("/", post(games_reversi_match))
      .route("/cancel", post(games_reversi_match_cancel))
    )
  )
}

async fn games_reversi_games() -> &'static str {
  "games_reversi_games"
}

async fn games_reversi_games_show() -> &'static str {
  "games_reversi_games_show"
}

async fn games_reversi_games_surrender() -> &'static str {
  "games_reversi_games_surrender"
}

async fn games_reversi_invitations() -> &'static str {
  "games_reversi_invitations"
}

async fn games_reversi_match() -> &'static str {
  "games_reversi_match"
}

async fn games_reversi_match_cancel() -> &'static str {
  "games_reversi_match_cancel"
}
