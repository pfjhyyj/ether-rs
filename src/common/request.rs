use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageRequest {
  pub current: i32,
  pub page_size: i32,
}
