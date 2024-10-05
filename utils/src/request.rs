use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageRequest {
    pub page: i32,
    pub size: i32,
}
