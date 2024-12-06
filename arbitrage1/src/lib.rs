use std::error::Error;
use tokio;

pub mod dex_client;
pub mod error;
pub mod pool;
pub mod price_monitor;
pub mod router_finder;
pub mod types;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct Config {
    pub network: NetworkConfig,
    pub trading: TradingConfig,
    pub risk: RiskConfig,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub rpc_url: String,
    pub ws_url: String,
}

#[derive(Debug, Clone)]
pub struct TradingConfig {
    pub min_profit_threshold: f64,
    pub max_trade_size: f64,
    pub slippage_tolerance: f64,
    pub timeout_blocks: u64,
}

#[derive(Debug, Clone)]
pub struct RiskConfig {
    pub max_daily_loss: f64,
    pub max_concurrent_trades: u32,
    pub circuit_breaker_threshold: f64,
} 