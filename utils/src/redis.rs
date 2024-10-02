use std::sync::OnceLock;

use crate::env;

type RedisPool = r2d2::Pool<redis::Client>;

static REDIS_POOL: OnceLock<RedisPool> = OnceLock::new();

pub async fn init_redis() {
  let url = env::get_env::<String>("REDIS_URL");

  let client = redis::Client::open(url)
    .unwrap_or_else(|e| panic!("Failed to connect to redis: {}", e));

  let mut conn = client.get_connection()
    .unwrap_or_else(|e| panic!("Failed to connect to redis: {}", e));

  let _ = redis::cmd("PING")
    .query::<String>(&mut conn)
    .unwrap_or_else(|e| panic!("Failed to connect to redis: {}", e));

  let pool = r2d2::Pool::builder()
    .max_size(20)
    .build(client.clone())
    .unwrap_or_else(|e| panic!("Failed to connect to redis: {}", e));

  let _ = REDIS_POOL.set(pool);  
}

pub fn redis_pool() -> &'static RedisPool {
  REDIS_POOL
    .get()
    .unwrap_or_else(|| panic!("Redis pool is not initiated"))
}
