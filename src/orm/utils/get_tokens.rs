use std::collections::HashSet;
use rbatis::Rbatis;
use crate::{
    Token,
    Chain,
    orm::model::TokenModel,
};


pub async fn get_tokens<'a, 'b>(db: &'b mut Rbatis, chains: &'a Vec<Chain>) -> Result<HashSet<Token<'a>>, Box<dyn std::error::Error>> {
    let mut tokens = HashSet::new();

    let stored_tokens = TokenModel::select_all_active_with_market_data(db, "token").await?;
    // println!("{:?}", stored_tokens);

    for stored_token in stored_tokens.iter() {
        tokens.insert(Token {
            ticker: stored_token.ticker.clone(),
            chain: chains.iter().find(|c| c.name == stored_token.chain.clone()).unwrap(),
            decimals: stored_token.decimals.clone(),
            name: stored_token.name.clone(),
            address: stored_token.address.clone(),
            price_usd: match &stored_token.price_usd {
                Some(p) => Some(format!("{p}")),
                None => None,
            },
            volume_24h_usd: match &stored_token.volume_24h_usd {
                Some(v) => Some(format!("{v}")),
                None => None,
            },
            market_provider_id: stored_token.market_provider_id.clone(),
        });
    }

    Ok(tokens)
}
