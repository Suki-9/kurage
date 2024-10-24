mod server;
mod utils;

#[tokio::main]
async fn main() {
  server::lunch_server().await;
}
