use std::{ sync::Arc };
use polynomial::{ fetcher::{ DataEngine } };
mod common;
use common::Result;
#[tokio::main]
async fn main() -> Result<()> {
    let data_engine = Arc::new(DataEngine::new());
    data_engine.start();
    tokio::signal::ctrl_c().await.expect("fail to listen to event");
    Ok(())
}
