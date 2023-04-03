use log::LevelFilter;
use simple_logger::SimpleLogger;
use hw3_testcontainers::account::server::run_account_server;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();

    run_account_server("exchange_server:1337".to_string(), 4321)
        .await;
}
