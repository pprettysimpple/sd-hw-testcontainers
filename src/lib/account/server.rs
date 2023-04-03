use hyper::{Body, Request, Response};

use log::info;

use crate::account::context::Context;
use crate::common::server::{get_from_params, run_server, ser_account, ser_shares, ser_str};

async fn handle_impl(mut context: Context, req: Request<Body>) -> Result<Response<Body>, String> {
    let uri = req.uri();

    info!("Got request with path: {} and params: {:?}", uri.path(), uri.query());
    match uri.path() {
        "/create_account" => {
            let name = get_from_params("name", &uri)?;
            let id = context.create_account(name, 0.0).await?;
            Ok(Response::new(Body::from(serde_json::to_string(&id).map_err(|e| e.to_string())?)))
        }
        "/get_account" => {
            let id = get_from_params("account_id", &uri)?;
            let account = context.get_account(id).await?;
            Ok(Response::new(Body::from(ser_account(account)?)))
        }
        "/add_balance" => {
            let id = get_from_params("account_id", &uri)?;
            let amount = get_from_params("amount", &uri)?;
            context.add_balance(id, amount).await?;
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        "/buy_share" => {
            let id = get_from_params("account_id", &uri)?;
            let share_id = get_from_params("share_id", &uri)?;
            let cost = get_from_params("cost", &uri)?;
            context.buy_share(id, share_id, cost).await?;
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        "/sell_share" => {
            let id = get_from_params("account_id", &uri)?;
            let share_id = get_from_params("share_id", &uri)?;
            let amount = get_from_params("amount", &uri)?;
            context.sell_share(id, share_id, amount).await?;
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        "/get_capitalization" => {
            let account_id = get_from_params("account_id", &uri)?;
            let capitalization = context.get_capitalization(account_id).await?;
            Ok(Response::new(Body::from(serde_json::to_string(&capitalization).map_err(|e| e.to_string())?)))
        }
        "admin/health" => {
            Ok(Response::new(Body::from(ser_str("OK".to_string())?)))
        }
        "/get_shares" => {
            let shares = context.get_shares().await?;
            Ok(Response::new(Body::from(ser_shares(shares)?)))
        }
        _ => {
            Ok(Response::new(Body::from(ser_str("Not found".to_string())?)))
        }
    }
}

pub async fn run_account_server(exchange_host: String, port: u16) {
    let context = Context::new(exchange_host);

    run_server(context, handle_impl, port).await;
}