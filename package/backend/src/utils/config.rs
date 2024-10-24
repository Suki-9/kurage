use once_cell::sync::Lazy;
use serde_derive::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub APP_PORT: i32,
}

pub static APP_CONFIG: Lazy<Config> = Lazy::new(|| match serde_yaml::from_str(&std::fs::read_to_string("./config/default.yaml").unwrap()) {
  Ok(c) => c,
  Err(_) => panic!("Failed to load Config file."),
});
