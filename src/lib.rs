// src/lib.rs
pub mod common;
pub mod context;
pub mod fetcher;
pub mod bot_error;
pub mod stream;
pub mod types;
pub mod errors;

pub mod prelude {
    pub use crate::common::EVENT_URL;
    pub use crate::common::MARKET_URL;
    pub use crate::common::CRYPTO_PATTERNS;
    pub use crate::common::WEBSOCKET_MARKET_URL;
    pub use crate::common::Market;
    pub use crate::common::Token;
    pub use crate::common::Result;
    pub use crate::stream::{ WebSocketStream };
    pub use crate::types::{ WssAuth, WssChannelType };
    pub use crate::context::BotContext;
}
