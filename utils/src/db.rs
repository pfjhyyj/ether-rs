use std::{sync::OnceLock, time::Duration};

use sea_orm::{ConnectOptions, Database as SeaOrmDatabase};
pub use sea_orm::DatabaseConnection;

use crate::env;

static DB_CONNECTION: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn init_db() {
  let url = env::get_env::<String>("DATABASE_URL");

  let opt = ConnectOptions::new(url)
    .max_connections(100)
    .max_connections(100)
    .min_connections(5)
    .connect_timeout(Duration::from_secs(5))
    .acquire_timeout(Duration::from_secs(5))
    .idle_timeout(Duration::from_secs(100))
    .max_lifetime(Duration::from_secs(100))
    .to_owned();

  let conn = SeaOrmDatabase::connect(opt)
    .await
    .unwrap_or_else(|e| panic!("Failed to connect to the database: {}", e));

  let _ = conn.ping()
    .await
    .is_err_and(|e| panic!("Failed to connect to the database: {}", e));
  
  let _ = DB_CONNECTION.set(conn);
}

pub fn conn() -> &'static DatabaseConnection {
  DB_CONNECTION.get().unwrap_or_else(|| panic!("Database connection is not initiated"))
}
