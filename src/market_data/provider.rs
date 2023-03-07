use web3::types::Address;
use std::collections::HashSet;
use std::fmt::Debug;


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenMarketData {
    pub ticker: String,
    pub token_id: String,
    pub price_usd: String,
    pub volume_24: String,
}

#[async_trait::async_trait]
pub trait MarketDataProvider: Debug + Sync {
    async fn cache_tokens_md(&mut self, addresses: HashSet<Address>) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_by_address(&self, address: Address) -> Result<Option<TokenMarketData>, Box<dyn std::error::Error>>;
}
