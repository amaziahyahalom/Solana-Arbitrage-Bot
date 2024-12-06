use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArbitrageError {
    #[error("Insufficient liquidity for token pair")]
    InsufficientLiquidity(String),

    #[error("Slippage exceeded: expected {expected}, actual {actual}")]
    SlippageExceeded { expected: f64, actual: f64 },

    #[error("Transaction timeout: {0}")]
    TransactionTimeout(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },
} 