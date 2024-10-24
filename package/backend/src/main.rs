mod server;
mod utils;

use utils::config;

#[tokio::main]
async fn main() {
  server::lunch_server(config::APP_CONFIG.APP_PORT).await;
}
