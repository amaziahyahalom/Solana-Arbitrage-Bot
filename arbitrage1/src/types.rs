use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub path: Vec<TokenPair>,
    pub expected_profit: f64,
    pub confidence_score: f64,
    pub execution_time_estimate: u64,
}

#[derive(Debug, Clone)]
pub struct Quote {
    pub input_amount: u64,
    pub output_amount: u64,
    pub price_impact: f64,
    pub fee: u64,
}

#[derive(Debug)]
pub enum TransactionState {
    Pending,
    Executing,
    Completed(Signature),
    Failed(String),
    Timeout,
}

#[derive(Debug)]
pub struct PriceData {
    pub token_pair: TokenPair,
    pub price: f64,
    pub timestamp: i64,
    pub volume: f64,
}

pub struct PriceQueue {
    prices: VecDeque<PriceData>,
    max_size: usize,
}

impl PriceQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            prices: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn push(&mut self, price_data: PriceData) {
        if self.prices.len() >= self.max_size {
            self.prices.pop_front();
        }
        self.prices.push_back(price_data);
    }
} 