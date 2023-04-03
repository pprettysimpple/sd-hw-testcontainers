use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl};
use hw3_testcontainers::common::repl::{AppError, get_param};
use hw3_testcontainers::common::repl::AppError::StringError;
use hw3_testcontainers::common::share::Share;
use hw3_testcontainers::exchange::client::ExchangeClient;

struct AppContext {
    client: ExchangeClient,
}

async fn add_share(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let share = Share {
        id: get_param(&args, "share_id")?,
        company_id: get_param(&args, "company_id")?,
        price: get_param(&args, "price")?,
        supply: get_param(&args, "supply")?,
        version: 0,
    };

    match context.client.add_share(share).await {
        Ok(val) => Ok(Some(format!("Share added: {}", val))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn get_shares(_args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    match context.client.get_shares().await {
        Ok(shares) => Ok(Some(format!("Shares: {:?}", shares))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn get_shares_by_company(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let company_id = get_param(&args, "company_id")?;

    match context.client.get_shares_by_company(company_id).await {
        Ok(shares) => Ok(Some(format!("Shares: {:?}", shares))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn set_share_price(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let share_id = get_param(&args, "share_id")?;
    let price = get_param(&args, "price")?;

    match context.client.set_share_price(share_id, price).await {
        Ok(val) => Ok(Some(format!("Share price set: {}", val))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let context = AppContext {
        client: ExchangeClient::new("localhost:1337".to_string()),
    };

    let mut repl = Repl::new(context)
        .with_name("Exchange Manager")
        .with_version("v0.1.0")
        .with_prompt("Ready")
        .with_command_async(
            Command::new("get-shares")
                .about("Returns all available shares"),
            |args, context| Box::pin(get_shares(args, context)),
        )
        .with_command_async(
            Command::new("get-shares-by-company")
                .arg(Arg::new("company_id").required(true))
                .about("Returns all shares of given company"),
            |args, context| Box::pin(get_shares_by_company(args, context)),
        )
        .with_command_async(
            Command::new("add-share")
                .arg(Arg::new("share_id").required(true))
                .arg(Arg::new("company_id").required(true))
                .arg(Arg::new("price").required(true))
                .arg(Arg::new("supply").required(true))
                .about("Adds share to the exchange"),
            |args, context| Box::pin(add_share(args, context)),
        )
        .with_command_async(
            Command::new("set-share-price")
                .arg(Arg::new("share_id").required(true))
                .arg(Arg::new("price").required(true))
                .about("Sets share price"),
            |args, context| Box::pin(set_share_price(args, context)),
        );

    match repl.run_async().await {
        Ok(_) => Ok(()),
        Err(e) => Err(StringError(format!("Repl.run Error: {}", e))),
    }
}