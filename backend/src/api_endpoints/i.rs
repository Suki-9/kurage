use axum::{
  routing::post,
  Router,
};

pub fn i_routes() -> Router {
  Router::new()
    .route("/", post(i))
    .route("/apps", post(i_apps))
    .route("/authorized-apps", post(i_authorized_apps))
    .route("/change-password", post(i_change_password))
    .route("/delete-account", post(i_delete_account))
    .route("/export-blocking", post(i_export_blocking))
    .route("/export-following", post(i_export_following))
    .route("/export-mute", post(i_export_mute))
    .route("/export-notes", post(i_export_notes))
    .route("/export-user-lists", post(i_export_user_lists))
    .route("/favorites", post(i_favorites))
    .nest("/gallery", Router::new()
      .route("/likes", post(i_gallery_likes))
      .route("/posts", post(i_gallery_posts))
    )
    .route("/import-following", post(i_import_following))
    .route("/import-user-lists", post(i_import_user_lists))
    .route("/move", post(i_move))
    .route("/notifications", post(i_notifications))
    .route("/page-likes", post(i_page_likes))
    .route("/pages", post(i_pages))
    .route("/pin", post(i_pin))
    .route("/read-all-messaging-messages", post(i_read_all_messaging_messages))
    .route("/read-all-unread-notes", post(i_read_all_unread_notes))
    .route("/read-announcement", post(i_read_announcement))
    .route("/regenerate-token", post(i_regenerate_token))
    .nest("/registry", Router::new()
      .route("/get-detail", post(i_registry_get_detail))
      .route("/get", post(i_registry_get))
      .route("/keys-with-type", post(i_registry_keys_with_type))
      .route("/keys", post(i_registry_keys))
      .route("/remove", post(i_registry_remove))
      .route("/set", post(i_registry_set))
    )
    .route("/revoke-token", post(i_revoke_token))
    .route("/signin-history", post(i_signin_history))
    .route("/unpin", post(i_unpin))
    .route("/update-email", post(i_update_email))
    .route("/update", post(i_update))
    .route("/user-group-invites", post(i_user_group_invites))
    .nest("/2fa", Router::new()
      .route("/done", post(i_2fa_done))
      .route("/key-done", post(i_2fa_key_done))
      .route("/password-less", post(i_2fa_password_less))
      .route("/register-key", post(i_2fa_register_key))
      .route("/register", post(i_2fa_register))
      .route("/remove-key", post(i_2fa_remove_key))
      .route("/unregister", post(i_2fa_unregister))
    )
}

async fn i() -> &'static str {
  "i"
}

async fn i_apps() -> &'static str {
  "i_apps"
}

async fn i_authorized_apps() -> &'static str {
  "i_authorized_apps"
}

async fn i_change_password() -> &'static str {
  "i_change_password"
}

async fn i_delete_account() -> &'static str {
  "i_delete_account"
}

async fn i_export_blocking() -> &'static str {
  "i_export_blocking"
}

async fn i_export_following() -> &'static str {
  "i_export_following"
}

async fn i_export_mute() -> &'static str {
  "i_export_mute"
}

async fn i_export_notes() -> &'static str {
  "i_export_notes"
}

async fn i_export_user_lists() -> &'static str {
  "i_export_user_lists"
}

async fn i_favorites() -> &'static str {
  "i_favorites"
} 

async fn i_gallery_likes() -> &'static str {
  "i_gallery_likes"
}

async fn i_gallery_posts() -> &'static str {
  "i_gallery_posts"
}

async fn i_import_following() -> &'static str {
  "i_import_following"
}

async fn i_import_user_lists() -> &'static str {
  "i_import_user_lists"
}

async fn i_move() -> &'static str {
  "i_move"
}

async fn i_notifications() -> &'static str {
  "i_notifications"
}

async fn i_page_likes() -> &'static str {
  "i_page_likes"
}

async fn i_pages() -> &'static str {
  "i_pages"
}

async fn i_pin() -> &'static str {
  "i_pin"
}

async fn i_read_all_messaging_messages() -> &'static str {
  "i_read_all_messaging_messages"
}

async fn i_read_all_unread_notes() -> &'static str {
  "i_read_all_unread_notes"
}

async fn i_read_announcement() -> &'static str {
  "i_read_announcement"
}

async fn i_regenerate_token() -> &'static str {
  "i_regenerate_token"
}

async fn i_registry_get_detail() -> &'static str {
  "i_registry_get_detail"
}

async fn i_registry_get() -> &'static str {
  "i_registry_get"
}

async fn i_registry_keys_with_type() -> &'static str {
  "i_registry_keys_with_type"
}

async fn i_registry_keys() -> &'static str {
  "i_registry_keys"
}

async fn i_registry_remove() -> &'static str {
  "i_registry_remove"
}

async fn i_registry_set() -> &'static str {
  "i_registry_set"
}

async fn i_revoke_token() -> &'static str {
  "i_revoke_token"
}

async fn i_signin_history() -> &'static str {
  "i_signin_history"
}

async fn i_unpin() -> &'static str {
  "i_unpin"
}

async fn i_update_email() -> &'static str {
  "i_update_email"
}

async fn i_update() -> &'static str {
  "i_update"
}

async fn i_user_group_invites() -> &'static str {
  "i_user_group_invites"
}

async fn i_2fa_done() -> &'static str {
  "i_2fa_done"
}

async fn i_2fa_key_done() -> &'static str {
  "i_2fa_key_done"
}

async fn i_2fa_password_less() -> &'static str {
  "i_2fa_password_less"
}

async fn i_2fa_register_key() -> &'static str {
  "i_2fa_register_key"
}

async fn i_2fa_register() -> &'static str {
  "i_2fa_register"
}

async fn i_2fa_remove_key() -> &'static str {
  "i_2fa_remove_key"
}

async fn i_2fa_unregister() -> &'static str {
  "i_2fa_unregister"
}
