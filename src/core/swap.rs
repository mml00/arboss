use std::collections::HashMap;
use crate::{
    Pair,
    exchange::TExchangeProvider
};


#[derive(Debug)]
pub struct Swap<'a> {
    pub pair: &'a Pair<'a>,
    pub amount: u128,
    pub price: String,
    pub provider: TExchangeProvider<'a>,
}


pub async fn fetch_swaps<'a>(
    pairs: &'a Vec<Pair<'a>>,
    providers: &'a HashMap<&'a str, TExchangeProvider<'a>>,
    cross_chain: bool,
) -> Result<Vec<Swap<'a>>, Box<dyn std::error::Error>> {
    let mut swaps: Vec<Swap> = Vec::new();

    if cross_chain {
        let mut cross_chain_pairs: Vec<[&Pair; 2]> = Vec::new();
        for pair1 in pairs.iter() {
            for pair2 in pairs.iter() {
                if pair1 != pair2 &&
                    pair1.from.ticker == pair2.to.ticker &&
                    pair1.to.ticker == pair2.from.ticker &&
                    pair1.from.chain.name != pair2.from.chain.name {
                    cross_chain_pairs.push([pair1, pair2]);
                }
            }
        }
        println!("Cross chain pairs found: {:?}", cross_chain_pairs.len());

        for c_pairs in cross_chain_pairs.iter() {
            let mut current_swaps = vec![];
            let amount_string = format!(
                "{}E{}",
                c_pairs[0].from.price_usd.as_ref().unwrap(),
                c_pairs[0].from.decimals.unwrap()
            );
            // println!("{amount_string}");
            let mut amount = amount_string.parse::<f64>().unwrap() * 100.; // 100$
            let initial_amount = amount.clone();
            for pair in c_pairs.iter() {
                for provider_name in pair.providers.as_ref().unwrap().iter() {
                    let provider = providers.get(provider_name.as_str());
                    let swap = provider.unwrap().get_swap(&pair, amount as u128).await?;
                    if let Some(s) = &swap {
                        amount = s.price.parse::<f64>().unwrap();
                    }
                    current_swaps.push(swap);
                }
            }
            // println!("{:#?}", current_swaps);
            if amount / initial_amount >= 1. && amount / initial_amount <= 1.4 {
                println!(
                    "({}[{}] <-> {}[{}]): {:.2}%",
                    c_pairs[0].from.ticker,
                    c_pairs[0].from.chain.name,
                    c_pairs[1].from.ticker,
                    c_pairs[1].from.chain.name,
                    (amount / initial_amount - 1.) * 100.);
            }
            // break
            // swaps.extend(current_swaps)
        }
    }
    Ok(swaps)
}
