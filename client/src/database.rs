

use std::time::Duration;

use sea_orm::{ConnectOptions, Database as SeaOrmDatabase};
pub use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;
use utils::env;

static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn get_db_connection() -> &'static DatabaseConnection {
  DB_CONNECTION
    .get_or_init(|| async {
      let host = env::get_env("POSTGRES_HOST");
      let port = env::get_env("POSTGRES_PORT");
      let user = env::get_env("POSTGRES_USER");
      let password = env::get_env("POSTGRES_PASSWORD");
      let database = env::get_env("POSTGRES_DB");
      let schema = env::get_env("POSTGRES_SCHEMA");
      let url = format!("postgres://{}:{}@{}:{}/{}?currentSchema={}", user, password, host, port, database, schema);

      let opt = ConnectOptions::new(url)
        .max_connections(100)
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(100))
        .max_lifetime(Duration::from_secs(100))
        .to_owned();

      return SeaOrmDatabase::connect(opt)
        .await
        .expect("Failed to connect to the database");
    })
    .await
}