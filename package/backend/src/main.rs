mod server;

#[tokio::main]
async fn main() {
  server::lunch_server(15400).await;
}
