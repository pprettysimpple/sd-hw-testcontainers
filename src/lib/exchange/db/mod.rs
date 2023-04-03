pub mod iface;
pub mod in_mem;

use std::sync::{Arc, Mutex};
use crate::common::share::Share;
use crate::exchange::db::iface::connector::ExchangeDBConnector;

#[derive(Clone, Debug)]
pub struct SyncDBWrapper<Impl> {
    db: Arc<Mutex<Impl>>,
}

impl<Impl> SyncDBWrapper<Impl> {
    pub fn new(db: Impl) -> Self {
        Self {
            db: Arc::new(Mutex::new(db)),
        }
    }
}

impl<Impl> ExchangeDBConnector for SyncDBWrapper<Impl>
    where Impl: ExchangeDBConnector {
    fn get_share(&self, id: String) -> Option<Share> {
        self.db.lock().unwrap().get_share(id)
    }

    fn get_shares(&self) -> Vec<Share> {
        self.db.lock().unwrap().get_shares()
    }

    fn get_shares_by_company(&self, company_id: String) -> Vec<Share> {
        self.db.lock().unwrap().get_shares_by_company(company_id)
    }

    fn add_share(&mut self, share: Share) -> Result<(), String> {
        self.db.lock().unwrap().add_share(share)
    }

    fn buy_share(&mut self, share_id: String, cost: f64) -> Result<f64, String> {
        self.db.lock().unwrap().buy_share(share_id, cost)
    }

    fn sell_share(&mut self, share_id: String, amount: f64) -> Result<f64, String> {
        self.db.lock().unwrap().sell_share(share_id, amount)
    }

    fn set_share_price(&mut self, share_id: String, price: f64) -> Result<(), String> {
        self.db.lock().unwrap().set_share_price(share_id, price)
    }
}