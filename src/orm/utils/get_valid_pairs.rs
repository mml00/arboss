use rbatis::Rbatis;
use std::collections::{
    HashSet
};
use crate::{
    Pair,
    Token,
    orm::model::{
        PairModel,
    },
};

pub async fn get_valid_pairs<'a, 'b>(
    db: &'b mut Rbatis,
    tokens: &'a HashSet<Token<'a>>
) -> Result<Vec<Pair<'a>>, Box<dyn std::error::Error>> {
    let mut pairs: Vec<Pair> = Vec::new();

    let stored_pairs = PairModel::select_all(db).await?;

    for pair in stored_pairs {
        let from_token = tokens.iter().find(|t| t.db_id.unwrap() == pair.from_token_id);
        let to_token = tokens.iter().find(|t| t.db_id.unwrap() == pair.to_token_id);
        if from_token.is_some() && to_token.is_some() {
            pairs.push(Pair {
                from: from_token.unwrap(),
                to: to_token.unwrap(),
                providers: Some(pair.providers.split(",").map(|p| p.to_string()).collect::<Vec<String>>()),
                pool_address: pair.pool_address.clone(),
                fee_percent: pair.fee_percent.clone(),
            });
        }
    }

    Ok(pairs)
}
