use crate::common::share::Share;

pub trait ExchangeDBConnector {
    fn get_share(&self, id: String) -> Option<Share>;
    fn get_shares(&self) -> Vec<Share>;
    fn get_shares_by_company(&self, company_id: String) -> Vec<Share>;

    fn add_share(&mut self, share: Share) -> Result<(), String>;

    fn buy_share(&mut self, share_id: String, cost: f64) -> Result<f64, String>;
    fn sell_share(&mut self, share_id: String, amount: f64) -> Result<f64, String>;

    fn set_share_price(&mut self, share_id: String, price: f64) -> Result<(), String>;
}