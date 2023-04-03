use crate::account::account::Account;
use crate::common::share::Share;
use crate::common::simple_client::{make_query, SimpleClient};

// this client is for the account service
pub struct AccountClient {
    pub client: SimpleClient,
}

impl AccountClient {
    pub fn new(host: String) -> AccountClient {
        AccountClient {
            client: SimpleClient::new(host),
        }
    }

    pub async fn create_account(&mut self, name: String) -> Result<String, String> {
        let uri = make_query(&self.client.host, "/create_account", format!("name={}", name))?;
        self.client.send_get_and_parse::<String>(uri).await
    }

    pub async fn get_account(&self, id: String) -> Result<Account, String> {
        let uri = make_query(&self.client.host, "/get_account", format!("account_id={}", id))?;
        self.client.send_get_and_parse::<Account>(uri).await
    }

    pub async fn add_balance(&mut self, id: String, amount: f64) -> Result<String, String> {
        let uri = make_query(&self.client.host, "/add_balance", format!("account_id={}&amount={}", id, amount))?;
        self.client.send_get_and_parse(uri).await
    }

    pub async fn buy_share(&mut self, id: String, share_id: String, cost: f64) -> Result<String, String> {
        let uri = make_query(&self.client.host, "/buy_share", format!("account_id={}&share_id={}&cost={}", id, share_id, cost))?;
        self.client.send_get_and_parse(uri).await
    }

    pub async fn sell_share(&mut self, id: String, share_id: String, amount: f64) -> Result<String, String> {
        let uri = make_query(&self.client.host, "/sell_share", format!("account_id={}&share_id={}&amount={}", id, share_id, amount))?;
        self.client.send_get_and_parse(uri).await
    }

    pub async fn get_capitalization(&self, account_id: String) -> Result<f64, String> {
        let uri = make_query(&self.client.host, "/get_capitalization", format!("account_id={}", account_id))?;
        self.client.send_get_and_parse::<f64>(uri).await
    }

    pub async fn get_all_available_shares(&self) -> Result<Vec<Share>, String> {
        let uri = make_query(&self.client.host, "/get_shares", "".to_string())?;
        self.client.send_get_and_parse::<Vec<Share>>(uri).await
    }
}
