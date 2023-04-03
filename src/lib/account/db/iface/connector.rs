use crate::account::account::Account;

pub trait AccountDBConnector {
    fn create_new_account(&mut self, account: Account) -> Result<(), String>;

    fn get_account(&self, id: String) -> Result<Account, String>;

    fn get_accounts(&self) -> Vec<Account>;

    fn add_balance(&mut self, account_id: String, amount: f64) -> Result<(), String>;

    fn change_amount(&mut self, account_id: String, share_id: String, amount: f64) -> Result<(), String>;

    fn get_shares(&self, account_id: String) -> Result<Vec<(String, f64)>, String>;

    fn get_share(&self, account_id: String, share_id: String) -> Result<(String, f64), String>;
}