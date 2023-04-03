use crate::common::share::Share;
use crate::common::simple_client::{make_query, SimpleClient};

#[derive(Clone, Debug)]
pub struct ExchangeClient {
    pub client: SimpleClient,
}

impl ExchangeClient {
    pub fn new(host: String) -> ExchangeClient {
        ExchangeClient {
            client: SimpleClient::new(host),
        }
    }

    pub async fn get_share(&self, id: String) -> Result<Share, String> {
        let uri = make_query(&self.client.host, "/get_share", format!("share_id={}", id))?;

        self.client.send_get_and_parse::<Share>(uri).await
    }

    pub async fn get_shares(&self) -> Result<Vec<Share>, String> {
        let uri = make_query(&self.client.host, "/get_shares", String::new())?;
        self.client.send_get_and_parse::<Vec<Share>>(uri).await
    }

    pub async fn get_shares_by_company(&self, company_id: String) -> Result<Vec<Share>, String> {
        let uri = make_query(&self.client.host, "/get_shares_by_company", format!("company_id={}", company_id))?;
        self.client.send_get_and_parse::<Vec<Share>>(uri).await
    }

    pub async fn add_share(&mut self, share: Share) -> Result<String, String> {
        let uri = make_query(&self.client.host, "/add_share", String::new())?;
        self.client.send_get_with_body_and_parse(uri, serde_json::to_string(&share).unwrap()).await
    }

    pub async fn buy_share(&mut self, share_id: String, cost: f64) -> Result<f64, String> {
        let uri = make_query(&self.client.host, "/buy_share", format!("share_id={}&cost={}", share_id, cost))?;
        self.client.send_get_and_parse::<f64>(uri).await
    }

    pub async fn sell_share(&mut self, share_id: String, amount: f64) -> Result<f64, String> {
        let uri = make_query(&self.client.host, "/sell_share", format!("share_id={}&amount={}", share_id, amount))?;
        self.client.send_get_and_parse::<f64>(uri).await
    }

    pub async fn set_share_price(&mut self, share_id: String, price: f64) -> Result<String, String> {
        let uri = make_query(&self.client.host, "/set_share_price", format!("share_id={}&price={}", share_id, price))?;
        self.client.send_get_and_parse::<String>(uri).await
    }
}