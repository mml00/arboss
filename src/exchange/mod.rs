pub mod oneinch;

use std::collections::HashMap;
use crate::{
    Chain,
    Token,
    utils::filters::Filter,
    market_data::{
        MarketDataProvider,
    },
};
use std::{
    fmt::Debug,
    collections::HashSet,
};


pub type TExchangeProvider<'a> = &'a dyn ExchangeProvider<'a>;
pub type TSimplePair<'a> = (&'a Token<'a>, &'a Token<'a>);

#[async_trait::async_trait]
pub trait ExchangeProvider<'a>: Debug + Sync {

    async fn get_tokens(
        &'a self,
        chains: &'a Vec<Chain>,
        filter: Option<&'a HashMap<String, Filter>>,
        market_data_provider: &'a Option<&'a mut dyn MarketDataProvider>,
    ) -> Result<HashSet<Token>, Box<dyn std::error::Error>>;

    async fn check_pairs_availability(
        &'a self,
        token_pairs: &'a Vec<TSimplePair>
    ) -> Result<Vec<TSimplePair>, Box<dyn std::error::Error>>;

    // async fn get_quotes(
    //     &'a self,
    //     pairs: &'a Vec<&'a Pair<'a>>
    // ) -> Result<Vec<Pair<'a>>, Box<dyn std::error::Error>>;
}

pub mod providers {
    pub use super::oneinch::OneInchProvider;
}
