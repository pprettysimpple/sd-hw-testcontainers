pub mod in_mem;
pub mod iface;

use std::sync::{Arc, Mutex};
use crate::account::account::Account;
use crate::account::db::iface::connector::AccountDBConnector;

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

impl<Impl> AccountDBConnector for SyncDBWrapper<Impl>
    where Impl: AccountDBConnector {
    fn create_new_account(&mut self, account: Account) -> Result<(), String> {
        self.db.lock().unwrap().create_new_account(account)
    }

    fn get_account(&self, id: String) -> Result<Account, String> {
        self.db.lock().unwrap().get_account(id)
    }

    fn get_accounts(&self) -> Vec<Account> {
        self.db.lock().unwrap().get_accounts()
    }

    fn add_balance(&mut self, account_id: String, amount: f64) -> Result<(), String> {
        self.db.lock().unwrap().add_balance(account_id, amount)
    }

    fn change_amount(&mut self, account_id: String, share_id: String, amount: f64) -> Result<(), String> {
        self.db.lock().unwrap().change_amount(account_id, share_id, amount)
    }

    fn get_shares(&self, account_id: String) -> Result<Vec<(String, f64)>, String> {
        self.db.lock().unwrap().get_shares(account_id)
    }

    fn get_share(&self, account_id: String, share_id: String) -> Result<(String, f64), String> {
        self.db.lock().unwrap().get_share(account_id, share_id)
    }
}