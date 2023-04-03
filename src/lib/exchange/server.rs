use hyper::{Body, Request, Response};

use log::{info};

use crate::common::server::{des_share, get_from_params, run_server, ser_share, ser_shares, ser_str};

use crate::exchange::db;
use crate::exchange::db::iface::connector::ExchangeDBConnector;

#[derive(Clone)]
struct Context {
    db: db::SyncDBWrapper<db::in_mem::InMemoryDB>,
}

async fn handle_impl(mut context: Context, req: Request<Body>) -> Result<Response<Body>, String> {
    let uri = req.uri();

    info!("Got request with path: {} and params: {:?}", uri.path(), uri.query());
    match uri.path() {
        "/get_shares" => {
            let shares = context.db.get_shares();
            Ok(Response::new(Body::from(ser_shares(shares).map_err(|e| e.to_string())?)))
        }
        "/get_share" => {
            let id = get_from_params("share_id", &uri)?;
            let share = context.db.get_share(id).ok_or("Failed to get share")?;
            Ok(Response::new(Body::from(ser_share(share).map_err(|e| e.to_string())?)))
        }
        "/get_shares_by_company" => {
            let company_id = get_from_params("company_id", &uri)?;
            let shares = context.db.get_shares_by_company(company_id);
            Ok(Response::new(Body::from(ser_shares(shares).map_err(|e| e.to_string())?)))
        }
        "/add_share" => {
            let body = hyper::body::to_bytes(req.into_body()).await.map_err(|e| e.to_string())?;
            let share = des_share(String::from_utf8(body.to_vec()).map_err(|e| e.to_string())?)?;
            context.db.add_share(share).map_err(|e| e.to_string())?;
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        "/buy_share" => {
            let id = get_from_params("share_id", &uri)?;
            let cost = get_from_params("cost", &uri)?;
            let amount = context.db.buy_share(id, cost)?;
            Ok(Response::new(Body::from(serde_json::to_string(&amount).map_err(|e| e.to_string())?)))
        }
        "/sell_share" => {
            let id = get_from_params("share_id", &uri)?;
            let amount = get_from_params("amount", &uri)?;
            let cost = context.db.sell_share(id, amount)?;
            Ok(Response::new(Body::from(serde_json::to_string(&cost).map_err(|e| e.to_string())?)))
        }
        "/set_share_price" => {
            let id = get_from_params("share_id", &uri)?;
            let price = get_from_params("price", &uri)?;
            context.db.set_share_price(id, price).map_err(|e| e.to_string())?;
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        "admin/health" => {
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        _ => {
            Ok(Response::new(Body::from(ser_str("Not found".to_string())?)))
        }
    }
}

pub async fn run_exchange_server(port: u16) {
    let context = Context {
        db: db::SyncDBWrapper::new(db::in_mem::InMemoryDB {
            shares: vec![],
        }),
    };

    run_server(context, handle_impl, port).await;
}