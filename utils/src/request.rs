use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PageRequest {
    pub page: Option<String>,
    pub size: Option<String>,
}

pub fn parse_page_request(req: PageRequest) -> (u64, u64) {
    let mut offset: u64 = 0;
    let mut limit: u64 = 20;

    if let Some(page) = req.page {
        let page = page.parse::<u64>().unwrap_or(1);
        if page < 1 {
            offset = 0;
        } else {
            offset = (page - 1) as u64 * limit;
        }
    }

    if let Some(size) = req.size {
        let size = size.parse::<u64>().unwrap_or(20);
        if size < 1 {
            limit = 20;
        } else if size > 100 {
            limit = 100;
        } else {
            limit = size as u64;
        }
    }

    (offset, limit)
}
