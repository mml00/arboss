use web3::types::Address;
use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    Chain,
    utils::filters::{
        Filter,
        execute_filter,
    },
    exchange::TExchangeProvider,
    market_data::{
        MarketDataProvider,
    },
};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<'a> {
    pub db_id: Option<u64>,
    pub ticker: String,
    pub chain: &'a Chain,
    pub decimals: Option<u8>,
    pub name: Option<String>,
    pub address: Option<Address>,
    pub price_usd: Option<String>,
    pub volume_24h_usd: Option<String>,
    pub market_provider_id: Option<String>,
}


impl<'a> Token<'a> {

    pub fn set_price(&mut self, price: f64) {
        self.price_usd = Some(price.to_string())
    }

    pub fn match_filter(&self, token_filters: &HashMap<String, Filter>) -> bool {
        let mut ok = true;

        for (field, filter) in token_filters.iter() {
            let empty = String::new();
            let value: String = match field.as_str() {
                "ticker" => self.ticker.clone(),
                "chain" => self.chain.name.clone(),
                "name" => self.name.as_ref().unwrap_or(&empty).clone(),
                "address" => match self.address {
                    Some(a) => format!("{:#x?}", a),
                    None => empty.clone()
                },
                _ => {
                    println!("Unknown filter field: {}", field);
                    empty.clone()
                }
            };
            if !execute_filter(&value, &filter) {
                ok = false;
                break
            }
        }
        ok
    }

}

pub async fn get_all_tokens<'a>(
    providers: &'a HashMap<&'a str, TExchangeProvider<'a>>,
    chains: &'a Vec<Chain>,
    filter: Option<&'a HashMap<String, Filter>>,
    market_data_provider: &'a Option<&'a mut dyn MarketDataProvider>,
) -> Result<HashSet<Token<'a>>, Box<dyn std::error::Error>> {

    let mut result_tokens: HashSet<Token> = HashSet::new();

    for (_, provider) in providers.iter() {
        let tokens = provider.get_tokens(&chains, filter, market_data_provider).await?;
        for token in tokens.iter() {
            result_tokens.insert(token.clone());
        }
    }

    Ok(result_tokens)
}
