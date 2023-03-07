use rbatis::Rbatis;
use rbdc::datetime::FastDateTime;
use crate::{
    Pair,
    orm::model::{
        PairModel,
        TokenModel,
    },
};


pub async fn update_pairs<'a>(db: &'a mut Rbatis, pairs: &'a Vec<Pair<'a>>) -> Result<(), Box<dyn std::error::Error>> {

    for pair in pairs.iter() {
        let from_token: TokenModel = if let Ok(tokens) = TokenModel::select_token(db, "token", pair.from.ticker.clone().to_uppercase(), pair.from.chain.name.clone()).await {
            if tokens.len() == 1 {tokens[0].clone()} else {println!("{}: Ticker not found or duplicated", pair.from.ticker.clone()); continue}
        } else {continue};
        let to_token: TokenModel = if let Ok(tokens) = TokenModel::select_token(db, "token", pair.to.ticker.clone().to_uppercase(), pair.to.chain.name.clone()).await {
            if tokens.len() == 1 {tokens[0].clone()} else {println!("{}: Ticker not found or duplicated", pair.from.ticker.clone()); continue}
        } else {continue};

        let stored_pair_candidates = PairModel::select_pair(db, "pair", from_token.id.unwrap(), to_token.id.unwrap()).await?;
        let mut updatable_pair = PairModel {
            id: None,
            from_token_id: from_token.id.unwrap(),
            to_token_id: to_token.id.unwrap(),
            providers: pair.providers.as_ref().unwrap().join(","),
            pool_address: pair.pool_address,
            fee_percent: pair.fee_percent.clone(),
            last_updated_at: FastDateTime::now(),
            is_active: true,
        };
        if stored_pair_candidates.len() == 0 {
            PairModel::insert(db, &updatable_pair).await?;
            continue;
        }
        if stored_pair_candidates.len() == 1 {
            updatable_pair.id = stored_pair_candidates[0].id;
            let _ = PairModel::update_by_column(db, &updatable_pair, "id").await?;
            continue;
        }
        println!("duplicate ticker found");
    // PairModel::insert(&mut db, &).await?;
    }

    // println!("{:?}", PairModel::select_pair(db, "pair", 7081, 7000).await?);

    // println!("{:?}", PairModel::select_all(&mut db).await?);
    Ok(())
}
