use crate::account::account::Account;
use crate::account::db::iface::connector::AccountDBConnector;

#[derive(Clone, Debug)]
pub(crate) struct InMemDB {
    accounts: Vec<Account>,
}

impl InMemDB {
    pub fn new() -> InMemDB {
        InMemDB { accounts: vec![] }
    }
}

impl AccountDBConnector for InMemDB {
    fn create_new_account(&mut self, account: Account) -> Result<(), String> {
        if self.accounts.iter().any(|a| a.id == account.id) {
            return Err(format!("Account with id {} already exists", account.id));
        }
        self.accounts.push(account);
        Ok(())
    }

    fn get_account(&self, id: String) -> Result<Account, String> {
        self
            .accounts
            .iter()
            .find(|a| a.id == id)
            .map(|val| val.clone())
            .ok_or(format!("Account with id {} does not exist", id))
    }

    fn get_accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    fn add_balance(&mut self, account_id: String, amount: f64) -> Result<(), String> {
        self.accounts
            .iter_mut()
            .find(|a| a.id == account_id)
            .map(|a| {
                a.balance += amount;
                Ok(())
            })
            .unwrap_or(Err(format!("Account with id {} does not exist", account_id)))
    }

    fn change_amount(&mut self, account_id: String, share_id: String, amount: f64) -> Result<(), String> {
        self.accounts
            .iter_mut()
            .find(|a| a.id == account_id)
            .map(|a| {
                match a.shares
                    .iter_mut()
                    .find(|(id, _)| *id == share_id) {
                    Some((share_id, share_amount)) => {
                        if *share_amount + amount < 0.0 {
                            return Err(format!("Account with id {} does not have enough shares with id {}", account_id, share_id));
                        }
                        *share_amount += amount;
                        Ok(())
                    }
                    None => {
                        if amount < 0.0 {
                            return Err(format!("Account with id {} does not have share with id {}", account_id, share_id));
                        }
                        a.shares.push((share_id, amount));
                        Ok(())
                    }
                }
            })
            .unwrap_or(Err(format!("Account with id {} does not exist", account_id)))
    }

    fn get_shares(&self, account_id: String) -> Result<Vec<(String, f64)>, String> {
        self.accounts
            .iter()
            .find(|a| a.id == account_id)
            .map(|a| a.shares.clone())
            .ok_or(format!("Account with id {} does not exist", account_id))
    }

    fn get_share(&self, account_id: String, share_id: String) -> Result<(String, f64), String> {
        self.accounts
            .iter()
            .find(|a| a.id == account_id)
            .map(|a| {
                a.shares
                    .iter()
                    .find(|s| s.0 == share_id)
                    .map(|val| val.clone())
                    .ok_or(format!("Account with id {} does not have share with id {}", account_id, share_id))
            })
            .unwrap_or(Err(format!("Account with id {} does not exist", account_id)))
    }
}