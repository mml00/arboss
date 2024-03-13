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

    let mut pairs: HashSet<TSimplePair<'a>> = HashSet::new();
    // let mut crosschain_pairs: Vec<TSimplePair<'a>> = Vec::new();

    // let mut crosschain_pairs: HashSet<TSimplePair<'a>> = HashSet::new();
    if cross_chain {
        let mut tokens_by_ticker = HashMap::new();
        let mut tokens_by_chain = HashMap::new();

        for token in tokens.iter() {
            if !tokens_by_ticker.contains_key(&token.ticker) {
                tokens_by_ticker.insert(token.ticker.clone(), vec![token]);
            } else {
                let ticker_tokens_vec = tokens_by_ticker.get_mut(&token.ticker).unwrap();
                ticker_tokens_vec.push(token);
            }

            if !tokens_by_chain.contains_key(&token.chain.name) {
                tokens_by_chain.insert(token.chain.name.clone(), vec![token]);
            } else {
                let chain_tokens_vec = tokens_by_chain.get_mut(&token.chain.name).unwrap();
                chain_tokens_vec.push(token);
            }
        }
        // let all_cross_chain_tokens = tokens_by_ticker
        //     .values()
        //     .fold(vec![], |acc, t| if t.len() > 1 {acc.extend(t); acc} else {acc})
        //     .collect::<Vec<_>>();

        for (_, tokens) in tokens_by_ticker.iter() {
            if tokens.len() > 1 {
                for token1 in tokens.iter() {
                    for token2 in tokens_by_chain.get(&token1.chain.name).unwrap().iter() {
                        if token1 != token2 && tokens_by_ticker.get(&token2.ticker).unwrap().len() > 1{
                            pairs.insert((token1, token2));
                        }
                    }
                }
            }
        }

        // for pair in crosschain_pairs.iter() {
        //     if token.chain.name != pair.0.chain.name || token.chain.name != pair.1.chain.name {continue}
        //     possible_pairs.push((token, pair.0));
        //     possible_pairs.push((pair.0, token));
        // }
    } else {
        for from_token in tokens.iter() {
            for to_token in tokens.iter() {
                if from_token == to_token {continue}

                // if cross_chain {
                //     if from_token.ticker == to_token.ticker && from_token.chain.name != to_token.chain.name {
                //         // crosschain_pairs.push((from_token, to_token))
                //     }
                // } else {
                if from_token.chain.name == to_token.chain.name {
                    pairs.insert((from_token, to_token));
                }
                // }
            }
        }
    }

    pairs.into_iter().collect::<Vec<_>>()
}


// TODO Rewrite
pub async fn validate_pairs<'a>(
    pairs_enumerations: &'a Vec<TSimplePair<'a>>,
    providers: &'a HashMap<&'a str, TExchangeProvider<'a>>,
    _cross_chain: bool,
) -> Result<Vec<Pair<'a>>, Box<dyn std::error::Error>> {

    let mut validated_pairs_with_providers = HashMap::new();

    for (provider_name, provider) in providers.iter() {
        let available_pairs = provider.check_pairs_availability(pairs_enumerations).await?;

        for pair in available_pairs.into_iter() {
            let pair_providers = validated_pairs_with_providers.entry(pair).or_insert(vec![]);
            pair_providers.push(String::from(*provider_name))
        }
    }

    let mut validated_pairs: Vec<Pair> = Vec::new();
    for (pair, providers) in validated_pairs_with_providers.iter() {
        validated_pairs.push(Pair {
            from: pair.0,
            to: pair.1,
            providers: Some((providers.clone()).to_vec()),
            pool_address: None,
            fee_percent: None,
        })
    }

    Ok(validated_pairs)
}
