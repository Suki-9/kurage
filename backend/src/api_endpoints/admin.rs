use axum::{
  routing::post,
  Router,
};

pub fn admin_routes() -> Router {
  Router::new()
    .route("/abuse-user-report", post(admin_abuse_user_report))
    .route("/delete-all-files-of-a-user", post(admin_delete_all_files_of_a_user))
    .route("/unset-user-avatar", post(admin_unset_user_avatar))
    .route("/unset-user-banner", post(admin_unset_user_banner))
    .route("/delete-logs", post(admin_delete_logs))
    .route("/get-index-stats", post(admin_get_index_stats))
    .route("/get-table-stats", post(admin_get_table_stats))
    .route("/invite", post(admin_invite))
    .route("/logs", post(admin_logs))
    .route("/meta", post(admin_meta))
    .route("/reset-password", post(admin_reset_password))
    .route("/resolve-abuse-user-report", post(admin_resolve_abuse_user_report))
    .route("/resync-chart", post(admin_resync_chart))
    .route("/send-email", post(admin_send_email))
    .route("/server-info", post(admin_server_info))
    .route("/show-moderation-logs", post(admin_show_moderation_logs))
    .route("/show-user", post(admin_show_user))
    .route("/show-users", post(admin_show_users))
    .route("/silence-user", post(admin_silence_user))
    .route("/unsilence-user", post(admin_unsilence_user))
    .route("/suspend-user", post(admin_suspend_user))
    .route("/unsuspend-user", post(admin_unsuspend_user))
    .route("/update-meta", post(admin_update_meta))
    .route("/vacuum", post(admin_vacuum))
    .nest("/accounts", Router::new()
      .route("/create", post(admin_accounts_create))
    )
    .nest("/ad", Router::new()
      .route("/create", post(admin_ad_create))
      .route("/delete", post(admin_ad_delete))
      .route("/list", post(admin_ad_list))
      .route("/update", post(admin_ad_update))
    )
    .nest("/announcements", Router::new()
      .route("/create", post(admin_announcements_create))
      .route("/delete", post(admin_announcements_delete))
      .route("/list", post(admin_announcements_list))
      .route("/update", post(admin_announcements_update))
    )
    .nest("/drive", Router::new()
      .route("/clean-remote-files", post(admin_drive_clean_remote_files))
      .route("/cleanup", post(admin_drive_cleanup))
      .route("/files", post(admin_drive_files))
      .route("/show-file", post(admin_drive_show_file))
    )
    .nest("/emoji", Router::new()
      .route("/add", post(admin_emoji_add))
      .route("/copy", post(admin_emoji_copy))
      .route("/list-remote", post(admin_emoji_list_remote))
      .route("/list", post(admin_emoji_list))
      .route("/remove", post(admin_emoji_remove))
      .route("/update", post(admin_emoji_update))
    )
    .nest("/federation", Router::new()
      .route("/delete-all-files", post(admin_federation_delete_all_files))
      .route("/refresh-remote-instance-metadata", post(admin_federation_refresh_remote_instance_metadata))
      .route("/remove-all-following", post(admin_federation_remove_all_following))
      .route("/update-instance", post(admin_federation_update_instance))
    )
    .nest("/invite", Router::new()
      .route("/create", post(admin_invite_create))
      .route("/list", post(admin_invite_list)
    ))
    .nest("/moderators", Router::new()
      .route("/add", post(admin_moderators_add))
      .route("/remove", post(admin_moderators_remove))
    )
    .nest("/promo", Router::new()
      .route("/create", post(admin_promo_create))
    )
    .nest("/queue", Router::new()
      .route("/clear", post(admin_queue_clear))
      .route("/deliver-delayed", post(admin_queue_deliver_delayed))
      .route("/inbox-delayed", post(admin_queue_inbox_delayed))
      .route("/jobs", post(admin_queue_jobs))
      .route("/stats", post(admin_queue_stats))
    )
    .nest("/relays", Router::new()
      .route("/add", post(admin_relays_add))
      .route("/list", post(admin_relays_list))
      .route("/remove", post(admin_relays_remove))
    )
}

async fn admin_abuse_user_report() -> &'static str {
  "admin_abuse_user_report"
}

async fn admin_delete_all_files_of_a_user() -> &'static str {
  "admin_delete_all_files_of_a_user"
}

async fn admin_unset_user_avatar() -> &'static str {
  "admin_unset_user_avatar"
}

async fn admin_unset_user_banner() -> &'static str {
  "admin_unset_user_banner"
}

async fn admin_delete_logs() -> &'static str {
  "admin_delete_logs"
}

async fn admin_get_index_stats() -> &'static str {
  "admin_get_index_stats"
}

async fn admin_get_table_stats() -> &'static str {
  "admin_get_table_stats"
}

async fn admin_invite() -> &'static str {
  "admin_invite"
}

async fn admin_logs() -> &'static str {
  "admin_logs"
}

async fn admin_meta() -> &'static str {
  "admin_meta"
}

async fn admin_reset_password() -> &'static str {
  "admin_reset_password"
}

async fn admin_resolve_abuse_user_report() -> &'static str {
  "admin_resolve_abuse_user_report"
}

async fn admin_resync_chart() -> &'static str {
  "admin_resync_chart"
}

async fn admin_send_email() -> &'static str {
  "admin_send_email"
}

async fn admin_server_info() -> &'static str {
  "admin_server_info"
}

async fn admin_show_moderation_logs() -> &'static str {
  "admin_show_moderation_logs"
}

async fn admin_show_user() -> &'static str {
  "admin_show_user"
}

async fn admin_show_users() -> &'static str {
  "admin_show_users"
}

async fn admin_silence_user() -> &'static str {
  "admin_silence_user"
}

async fn admin_unsilence_user() -> &'static str {
  "admin_unsilence_user"
}

async fn admin_suspend_user() -> &'static str {
  "admin_suspend_user"
}

async fn admin_unsuspend_user() -> &'static str {
  "admin_unsuspend_user"
}

async fn admin_update_meta() -> &'static str {
  "admin_update_meta"
}

async fn admin_vacuum() -> &'static str {
  "admin_vacuum"
}

async fn admin_accounts_create() -> &'static str {
  "admin_accounts_create"
}

async fn admin_ad_create() -> &'static str {
  "admin_ad_create"
}

async fn admin_ad_delete() -> &'static str {
  "admin_ad_delete"
}

async fn admin_ad_list() -> &'static str {
  "admin_ad_list"
}

async fn admin_ad_update() -> &'static str {
  "admin_ad_update"
}

async fn admin_announcements_create() -> &'static str {
  "admin_announcements_create"
}

async fn admin_announcements_delete() -> &'static str {
  "admin_announcements_delete"
}

async fn admin_announcements_list() -> &'static str {
  "admin_announcements_list"
}

async fn admin_announcements_update() -> &'static str {
  "admin_announcements_update"
}

async fn admin_drive_clean_remote_files() -> &'static str {
  "admin_drive_clean_remote_files"
}

async fn admin_drive_cleanup() -> &'static str {
  "admin_drive_cleanup"
}

async fn admin_drive_files() -> &'static str {
  "admin_drive_files"
}

async fn admin_drive_show_file() -> &'static str {
  "admin_drive_show_file"
}

async fn admin_emoji_add() -> &'static str {
  "admin_emoji_add"
}

async fn admin_emoji_copy() -> &'static str {
  "admin_emoji_copy"
}

async fn admin_emoji_list_remote() -> &'static str {
  "admin_emoji_list_remote"
}

async fn admin_emoji_list() -> &'static str {
  "admin_emoji_list"
}

async fn admin_emoji_remove() -> &'static str {
  "admin_emoji_remove"
}

async fn admin_emoji_update() -> &'static str {
  "admin_emoji_update"
}

async fn admin_federation_delete_all_files() -> &'static str {
  "admin_federation_delete_all_files"
}

async fn admin_federation_refresh_remote_instance_metadata() -> &'static str {
  "admin_federation_refresh_remote_instance_metadata"
}

async fn admin_federation_remove_all_following() -> &'static str {
  "admin_federation_remove_all_following"
}

async fn admin_federation_update_instance() -> &'static str {
  "admin_federation_update_instance"
}

async fn admin_invite_create() -> &'static str {
  "admin_invite_create"
}

async fn admin_invite_list() -> &'static str {
  "admin_invite_list"
}

async fn admin_moderators_add() -> &'static str {
  "admin_moderators_add"
}

async fn admin_moderators_remove() -> &'static str {
  "admin_moderators_remove"
}

async fn admin_promo_create() -> &'static str {
  "admin_promo_create"
}

async fn admin_queue_clear() -> &'static str {
  "admin_queue_clear"
}

async fn admin_queue_deliver_delayed() -> &'static str {
  "admin_queue_deliver_delayed"
}

async fn admin_queue_inbox_delayed() -> &'static str {
  "admin_queue_inbox_delayed"
}

async fn admin_queue_jobs() -> &'static str {
  "admin_queue_jobs"
}

async fn admin_queue_stats() -> &'static str {
  "admin_queue_stats"
}

async fn admin_relays_add() -> &'static str {
  "admin_relays_add"
}

async fn admin_relays_list() -> &'static str {
  "admin_relays_list"
}

async fn admin_relays_remove() -> &'static str {
  "admin_relays_remove"
}
