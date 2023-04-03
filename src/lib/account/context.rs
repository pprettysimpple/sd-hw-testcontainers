use rand::distributions::Alphanumeric;
use rand::Rng;
use crate::account::account::Account;
use crate::account::db;
use crate::account::db::iface::connector::AccountDBConnector;
use crate::common::share::Share;
use crate::exchange::client::ExchangeClient;

#[derive(Clone, Debug)]
pub struct Context {
    db: db::SyncDBWrapper<db::in_mem::InMemDB>,
    client: ExchangeClient,
}

impl Context {
    pub fn new(exchange_host: String) -> Self {
        Self {
            db: db::SyncDBWrapper::new(db::in_mem::InMemDB::new()),
            client: ExchangeClient::new(exchange_host),
        }
    }

    pub async fn create_account(&mut self, name: String, balance: f64) -> Result<String, String> {
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        self.db.create_new_account(Account::new(id.clone(), name, balance))?;
        Ok(id)
    }

    pub async fn get_account(&self, id: String) -> Result<Account, String> {
        self.db.get_account(id)
    }

    pub async fn add_balance(&mut self, id: String, amount: f64) -> Result<(), String> {
        self.db.add_balance(id, amount)
    }

    pub async fn buy_share(&mut self, account_id: String, share_id: String, cost: f64) -> Result<(), String> {
        if cost > self.db.get_account(account_id.clone())?.balance {
            return Err(format!("Not enough money to buy share with id {}", share_id));
        }

        let amount = self.client.buy_share(share_id.clone(), cost).await?;
        self.db.change_amount(account_id.clone(), share_id, amount)?;
        self.db.add_balance(account_id, -cost)
    }

    pub async fn sell_share(&mut self, account_id: String, share_id: String, amount: f64) -> Result<(), String> {
        let cost = self.client.sell_share(share_id.clone(), amount).await?;
        self.db.change_amount(account_id.clone(), share_id, -amount)?;
        self.db.add_balance(account_id, cost)
    }

    pub async fn get_capitalization(&self, account_id: String) -> Result<f64, String> {
        let account = self.db.get_account(account_id)?;
        let mut capitalisation = account.balance;
        for (share_id, amount) in account.shares {
            capitalisation += self.client.get_share(share_id).await?.price * amount;
        }
        Ok(capitalisation)
    }

    // proxy methods for exchange client
    // get all available shares on exchange
    pub async fn get_shares(&self) -> Result<Vec<Share>, String> {
        self.client.get_shares().await
    }
}