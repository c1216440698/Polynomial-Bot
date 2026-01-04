use polyfill_rs::errors::PolyfillError;

pub const EVENT_URL: &str = "https://gamma-api.polymarket.com/events";
pub const SLUG_URL: &str = "https://gamma-api.polymarket.com/events/slug";
pub const MARKET_URL: &str = "https://gamma-api.polymarket.com/markets";
pub const SPORT_URL: &str = "https://gamma-api.polymarket.com/sports";

pub static CRYPTO_PATTERNS: &[&str] = &[
    "bitcoin-above",
    "ethereum-above",
    "bitcoin-up-or-down",
    "xrp-up-or-down",
    "ethereum-price",
    "xrp-price",
    "xrp-above",
    "solana-above",
    "ethereum-up-or-down",
];

pub type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketType {
    POLYMARKET,
    KALSHI,
}

#[derive(Debug, Clone, Hash)]
pub enum Outcome {
    YES,
    NO,
}
