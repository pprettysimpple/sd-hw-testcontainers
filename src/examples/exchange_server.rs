use log::LevelFilter;
use simple_logger::SimpleLogger;
use hw3_testcontainers::exchange::server::run_exchange_server;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .init()
        .unwrap();

    run_exchange_server(1337)
        .await;
}
