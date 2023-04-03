use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Share {
    pub id: String,
    pub company_id: String,
    pub price: f64,
    pub supply: f64,
    pub version: u64,
}

impl Share {
    pub fn new(id: String, company_id: String, price: f64, supply: f64) -> Share {
        Share {
            id,
            company_id,
            price,
            supply,
            version: 0,
        }
    }

    pub fn new_with_version(id: String, company_id: String, price: f64, supply: f64, version: u64) -> Share {
        Share {
            id,
            company_id,
            price,
            supply,
            version,
        }
    }
}