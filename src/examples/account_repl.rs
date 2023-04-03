use reedline_repl_rs::clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl};
use hw3_testcontainers::account::client::AccountClient;
use hw3_testcontainers::common::repl::{AppError, get_param};
use crate::AppError::StringError;

struct AppContext {
    client: AccountClient,
}

async fn create_account(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let name: String = get_param(&args, "name")?;

    match context.client.create_account(name.clone()).await {
        Ok(id) => Ok(Some(format!("Account {} created with id {}", name, id))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn get_account(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let id = get_param(&args, "id")?;

    match context.client.get_account(id).await {
        Ok(account) => Ok(Some(format!("Account: {:?}", account))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn add_balance(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let id = get_param(&args, "id")?;
    let amount = get_param(&args, "amount")?;

    match context.client.add_balance(id, amount).await {
        Ok(_) => Ok(Some(format!("Balance added"))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn buy_share(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let id = get_param(&args, "id")?;
    let share_id = get_param(&args, "share_id")?;
    let cost = get_param(&args, "cost")?;

    match context.client.buy_share(id, share_id, cost).await {
        Ok(_) => Ok(Some(format!("Share bought"))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn sell_share(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let id = get_param(&args, "id")?;
    let share_id = get_param(&args, "share_id")?;
    let amount = get_param(&args, "amount")?;

    match context.client.sell_share(id, share_id, amount).await {
        Ok(_) => Ok(Some(format!("Share sold"))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn get_capitalization(args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    let id = get_param(&args, "id")?;

    match context.client.get_capitalization(id).await {
        Ok(capitalization) => Ok(Some(format!("Capitalization: {}", capitalization))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

async fn get_shares(_args: ArgMatches, context: &mut AppContext) -> Result<Option<String>, AppError> {
    match context.client.get_all_available_shares().await {
        Ok(shares) => Ok(Some(format!("Shares: {:?}", shares))),
        Err(e) => Ok(Some(format!("Error: {}", e))),
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let context = AppContext {
        client: AccountClient::new("localhost:4321".to_string()),
    };

    let mut repl = Repl::new(context)
        .with_name("Account Manager")
        .with_version("v0.1.0")
        .with_prompt("Ready")
        .with_command_async(
            Command::new("create-account")
                .arg(Arg::new("name").required(true))
                .about("Creates an account with given name and returns its id (like password))"),
            |args, context| Box::pin(create_account(args, context)),
        )
        .with_command_async(
            Command::new("get-account")
                .arg(Arg::new("id").required(true))
                .about("Returns account with given id"),
            |args, context| Box::pin(get_account(args, context)),
        )
        .with_command_async(
            Command::new("add-balance")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("amount").required(true))
                .about("Adds balance to account with given id"),
            |args, context| Box::pin(add_balance(args, context)),
        )
        .with_command_async(
            Command::new("buy-share")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("share_id").required(true))
                .arg(Arg::new("cost").required(true))
                .about("Buys share with given id for given cost"),
            |args, context| Box::pin(buy_share(args, context)),
        )
        .with_command_async(
            Command::new("sell-share")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("share_id").required(true))
                .arg(Arg::new("amount").required(true))
                .about("Sells share with given id for given amount"),
            |args, context| Box::pin(sell_share(args, context)),
        )
        .with_command_async(
            Command::new("get-capitalization")
                .arg(Arg::new("id").required(true))
                .about("Returns capitalization of account with given id"),
            |args, context| Box::pin(get_capitalization(args, context)),
        )
        .with_command_async(
            Command::new("get-shares")
                .about("Returns all available shares"),
            |args, context| Box::pin(get_shares(args, context)));

    match repl.run_async().await {
        Ok(_) => Ok(()),
        Err(e) => Err(StringError(format!("Repl.run Error: {}", e))),
    }
}