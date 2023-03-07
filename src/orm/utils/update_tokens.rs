use rbatis::Rbatis;
use std::str::FromStr;
use rbdc::{
    decimal::Decimal,
    datetime::FastDateTime
};
use crate::{
    Token,
    orm::model::TokenModel
};


pub async fn update_tokens<'a>(db: &'a mut Rbatis, tokens: &'a Vec<&'a Token<'a>>) -> Result<(), Box<dyn std::error::Error>> {
    let mut active_tokens = TokenModel::select_all_active(db, "token").await?;

    for token in active_tokens.iter_mut() {
        token.is_active = false;
        let _ = TokenModel::update_by_column(db, token, "id").await?;
    }

    for token in tokens.iter() {
        let stored_token_candidates = TokenModel::select_token(db, "token", token.ticker.clone(), token.chain.name.clone()).await?;
        let mut updatable_token = TokenModel {
            id: None,
            ticker: token.ticker.clone(),
            chain: token.chain.name.clone(),
            decimals: token.decimals,
            name: token.name.clone(),
            address: token.address,
            market_provider_id: token.market_provider_id.clone(),
            price_usd: match &token.price_usd {
                Some(price) => Some(Decimal::from_str(price.as_str()).unwrap()),
                None => None,
            },
            volume_24h_usd: match &token.volume_24h_usd {
                Some(volume) => Some(Decimal::from_str(volume.as_str()).unwrap()),
                None => None,
            },
            last_updated_at: FastDateTime::now(),
            is_active: true,
        };

        if stored_token_candidates.len() == 0 {
            TokenModel::insert(db, &updatable_token).await?;
            continue;
        }
        if stored_token_candidates.len() == 1 {
            updatable_token.id = stored_token_candidates[0].id;
            let _ = TokenModel::update_by_column(db, &updatable_token, "id").await?;
            continue;
        }
        println!("duplicate ticker found");
    }

    // println!("Token objects found: {}", tokens.len());

    Ok(())
}
