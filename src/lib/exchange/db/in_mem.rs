use crate::common::share::Share;
use crate::exchange::db::iface::connector::ExchangeDBConnector;

#[derive(Clone, Debug)]
pub struct InMemoryDB {
    pub shares: Vec<Share>,
}

impl ExchangeDBConnector for InMemoryDB {
    fn get_share(&self, id: String) -> Option<Share> {
        self.shares.iter().find(|s| s.id == id).map(|val| val.clone())
    }

    fn get_shares(&self) -> Vec<Share> {
        self.shares.clone()
    }

    fn get_shares_by_company(&self, company_id: String) -> Vec<Share> {
        self.shares
            .iter()
            .filter(|s| s.company_id == company_id)
            .map(|val| val.clone())
            .collect()
    }

    // Adds if share with id does not exist
    fn add_share(&mut self, share: Share) -> Result<(), String> {
        if self.shares.iter().any(|s| s.id == share.id) {
            Err(format!("Share with id {} already exists", share.id))
        } else {
            self.shares.push(share);
            Ok(())
        }
    }

    fn buy_share(&mut self, share_id: String, cost: f64) -> Result<f64, String> {
        self.shares
            .iter_mut()
            .find(|s| s.id == share_id)
            .map(|s| {
                if s.price * s.supply < cost {
                    return Err(format!(
                        "Not enough money to buy share with id {}",
                        share_id
                    ));
                }
                let amount = cost / s.price;
                s.supply -= amount;
                Ok(amount)
            })
            .unwrap_or(Err(format!("Share with id {} does not exist", share_id)))
    }

    fn sell_share(&mut self, share_id: String, amount: f64) -> Result<f64, String> {
        self.shares
            .iter_mut()
            .find(|s| s.id == share_id)
            .map(|s| {
                s.supply += amount;
                Ok(amount * s.price)
            })
            .unwrap_or(Err(format!("Share with id {} does not exist", share_id)))
    }

    fn set_share_price(&mut self, share_id: String, price: f64) -> Result<(), String> {
        self.shares
            .iter_mut()
            .find(|s| s.id == share_id)
            .map(|s| {
                s.price = price;
                Ok(())
            })
            .unwrap_or(Err(format!("Share with id {} does not exist", share_id)))
    }
}