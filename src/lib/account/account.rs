use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub balance: f64,
    pub shares: Vec<(String, f64)>, // share id and amount
}

impl Account {
    pub fn new(id: String, name: String, balance: f64) -> Account {
        Account {
            id,
            name,
            balance,
            shares: vec![],
        }
    }
}