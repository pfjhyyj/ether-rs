use std::{env, fmt::Debug, str::FromStr};

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

pub fn get_env<T>(key: &str) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    let var = env::var(key).expect(&format!("{} must be set!", key));
    var.parse::<T>()
        .expect(&format!("{} can not be parse to correct type!", key))
}
