use std::env;

// Load the configuration from the .env files
// references:
// - https://github.com/dotenv-rs/dotenv/issues/41
// - https://github.com/allan2/dotenvy/issues/96
pub fn load_config() {
  let profile = if cfg!(test) {
    "test" 
  } else if cfg!(debug_assertions) {
    "development"
  } else {
    "production"
  };
  dotenvy::from_filename(format!(".env.{}.local", profile)).ok();
  dotenvy::from_filename(".env.local").ok();
  dotenvy::from_filename(format!(".env.{}", profile)).ok();
  dotenvy::dotenv().ok();
}

pub fn get_env(key: &str) -> String {
  env::var(key).expect(&format!("{} must be set", key))
}