use std::collections::{
    HashMap,
    HashSet,
};

use crate::{
    Token,
    Pair,
    exchange::{
        TExchangeProvider,
        TSimplePair,
    }
};


pub fn get_possible_pairs_enumerations<'a>(
    tokens: &'a HashSet<Token<'a>>,
    cross_chain: bool,
) -> Vec<TSimplePair<'a>> {

    let mut possible_pairs: Vec<TSimplePair<'a>> = Vec::new();
    let mut crosschain_pairs: Vec<TSimplePair<'a>> = Vec::new();

    for from_token in tokens.iter() {
        for to_token in tokens.iter() {
            if from_token == to_token {continue}

            if cross_chain {
                if from_token.ticker == to_token.ticker && from_token.chain.name != to_token.chain.name {
                    crosschain_pairs.push((from_token, to_token))
                }
            } else {
                if from_token.chain.name == to_token.chain.name {
                    possible_pairs.push((from_token, to_token))
                }
            }
        }
    }
    if cross_chain {
        for token in tokens.iter() {
            for pair in crosschain_pairs.iter() {
                possible_pairs.push((token, pair.0));
                possible_pairs.push((pair.0, token));
            }
        }
    }

    possible_pairs
}


// TODO Rewrite
pub async fn validate_pairs<'a>(
    pairs_enumerations: &'a Vec<TSimplePair<'a>>,
    providers: &'a HashMap<&'a str, TExchangeProvider<'a>>,
) -> Result<Vec<Pair<'a>>, Box<dyn std::error::Error>> {

    let mut validated_pairs_with_providers = HashMap::new();

    for (provider_name, provider) in providers.iter() {
        let available_pairs = provider.check_pairs_availability(pairs_enumerations).await?;

        for pair in available_pairs.into_iter() {
            let pair_providers = validated_pairs_with_providers.entry(pair).or_insert(vec![]);
            pair_providers.push(*provider_name)
        }
    }

    let mut validated_pairs: Vec<Pair> = Vec::new();
    for (pair, providers) in validated_pairs_with_providers.iter() {
        validated_pairs.push(Pair {
            from: pair.0,
            to: pair.1,
            providers: Some((*providers.clone()).to_vec()),
            pool_address: None,
            fee_percent: None,
        })
    }

    Ok(validated_pairs)
}
