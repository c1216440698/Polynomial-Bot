use dashmap::DashMap;
use std::sync::Arc;
use smol_str::SmolStr;
// Before: use polymarket_rs_client::{ClobClient, Side, OrderType};
use polyfill_rs::{ ClobClient, Side, OrderType };

enum MarketType {
    POLYMARKET,
    KALSHI,
}

#[derive(Debug, Clone)]
enum TokenType {
    YES,
    NO,
}

#[derive(Debug, Clone)]
struct Token {
    market_name: SmolStr,
    token_id: u64,
    token_type: TokenType,
}

struct BotContext {
    client: ClobClient,
    tokens: Arc<DashMap<MarketType, Vec<Token>>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ClobClient::new("https://clob.polymarket.com");
    let markets = client.get_sampling_markets(None).await?;
    println!("Found {} markets", markets.data.len());
    Ok(())
}
