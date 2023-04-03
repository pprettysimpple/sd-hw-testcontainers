use hw3_testcontainers::account::client::AccountClient;
use hw3_testcontainers::common::share::Share;
use hw3_testcontainers::exchange::client::ExchangeClient;

#[tokio::test]
async fn test_basic_pipeline() -> Result<(), String> {
    let mut account_client = AccountClient::new("localhost:4321".to_string());
    let mut exchange_client = ExchangeClient::new("localhost:1337".to_string());

    exchange_client.add_share(Share::new("share-id".to_string(), "company-id".to_string(), 10.0, 100.0)).await?;

    let account_id = account_client.create_account("account-name".to_string()).await?;
    account_client.add_balance(account_id.clone(), 100.0).await?;
    account_client.buy_share(account_id.clone(), "share-id".to_string(), 10.0).await?;

    let actual_cap = account_client.get_capitalization(account_id.clone()).await?;
    if actual_cap != 100.0 {
        return Err(format!("Expected capitalization to be 100.0, got {}", actual_cap));
    }

    exchange_client.set_share_price("share-id".to_string(), 20.0).await?;

    let actual_cap = account_client.get_capitalization(account_id.clone()).await?;
    if actual_cap != 110.0 {
        return Err(format!("Expected capitalization to be 110.0, got {}", actual_cap));
    }

    account_client.sell_share(account_id.clone(), "share-id".to_string(), 1.0).await?;

    let actual_cap = account_client.get_capitalization(account_id.clone()).await?;
    if actual_cap != 110.0 {
        return Err(format!("Expected capitalization to be 110.0, got {}", actual_cap));
    }

    let shares = exchange_client.get_shares().await?;
    assert_eq!(shares.len(), 1);
    assert_eq!(shares[0].id, "share-id");
    assert_eq!(shares[0].company_id, "company-id");
    assert_eq!(shares[0].price, 20.0);
    assert_eq!(shares[0].supply, 100.0);

    let account = account_client.get_account(account_id.clone()).await?;
    assert_eq!(account.balance, 110.0);

    Ok(())
}