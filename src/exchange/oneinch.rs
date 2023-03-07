use web3::types::Address;
use rand::{
    seq::SliceRandom,
    thread_rng
};
use futures::{
    stream::FuturesUnordered,
    StreamExt,
};
use std::io::{
    stdout,
    Write,
};

use crate::{
    Token,
    Chain,
    utils::filters::Filter,
    exchange::{
        ExchangeProvider,
        TSimplePair,
    },
    config::{
        CProxy,
    },
    market_data::{
        MarketDataProvider,
        CoingeckoMarketDataProvider,
    },
};

// response types dependencies
use reqwest::{
    Client,
    Proxy
};
use serde::Deserialize;
use std::collections::{
    HashMap,
    HashSet
};


// OneInch Token Response type
#[derive(Debug, Deserialize, Clone)]
struct OIRToken {
    symbol: String,
    name: String,
    address: Address,
    decimals: u8,
    // wrappedNative: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct OIRTokens {
    tokens: HashMap<String, OIRToken>,
}

// #[derive(Debug, Deserialize)]
// #[allow(non_snake_case)]
// struct OIRQuote {
//     fromToken: OIRToken,
//     toToken: OIRToken,
//     fromTokenAmount: String,
//     toTokenAmount: String,
//     // estimatedGas: String,
// }


#[derive(Debug)]
pub struct OneInchProvider {
    proxy_configs: Vec<CProxy>,
}

impl OneInchProvider {
    pub fn new(
        proxy_configs: Vec<CProxy>,
    ) -> Self {
        Self {
            // client: reqwest_client.build().unwrap(),
            proxy_configs
        }
    }
}

#[async_trait::async_trait]
impl<'a> ExchangeProvider<'a> for OneInchProvider {

    async fn check_pairs_availability(
        &'a self,
        token_pairs: &'a Vec<TSimplePair>
    ) -> Result<Vec<TSimplePair>, Box<dyn std::error::Error>> {

        let mut available_pairs: Vec<TSimplePair> = vec![];
        if token_pairs.len() == 0 {return Ok(available_pairs)};

        let mut pairs_iter = token_pairs.iter();

        let mut clients: Vec<Client> = Vec::new();
        let mut requests_queue = FuturesUnordered::new();

        for proxy in &self.proxy_configs {
            clients.push(
                Client::builder().proxy(
                    Proxy::https(&proxy.url)
                        .unwrap()
                        .basic_auth(&proxy.user, &proxy.password)
                ).build().unwrap()
            )
        }

        for _ in 1..40 {
            let pair = pairs_iter.next();
            requests_queue.push(get_quote(pair.unwrap(), clients.choose(&mut thread_rng()).unwrap()));
        }

        let mut i: u64 = 0;
        while let Some(result) = requests_queue.next().await {
            if let Some(pair) = result {
                available_pairs.push(pair);
            }
            i += 1;
            let _ = stdout().flush();
            if let Some(pair) = pairs_iter.next() {
                requests_queue.push(get_quote(pair, clients.choose(&mut thread_rng()).unwrap()));
            };
            print!("\r{i}/{}, Successful: {}", token_pairs.len(), available_pairs.len());
        }
        println!();

        Ok(available_pairs)
    }

    // async fn get_quotes(
    //     &'a self,
    //     pairs: &'a Vec<&'a Pair<'a>>
    // ) -> Result<Vec<Pair<'a>>, Box<dyn std::error::Error>> {
    //     let pairs: Vec<Pair> = vec![];

    //     Ok(pairs)
    // }
    // !!! https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
    // async fn get_quote(
    //     &self, token0: &'a Token<'a>, token1: &'a Token<'a>
    // ) -> Result<Vec<Pair<'a>>, Box<dyn std::error::Error>>;
    async fn get_tokens(
        &'a self,
        chains: &'a Vec<Chain>,
        filter: Option<&'a HashMap<String, Filter>>,
        #[allow(unused_variables)]
        market_data_provider: &'a Option<&'a mut dyn MarketDataProvider>,
    ) -> Result<HashSet<Token>, Box<dyn std::error::Error>> {

        let mut result_tokens = HashSet::new();

        for chain in chains.iter() {
            let tokens_response = loop {
                let resp = reqwest::get(format!("https://api.1inch.exchange/v5.0/{}/tokens", chain.id)).await?;
                if resp.status().is_success() {
                    match resp.json::<OIRTokens>().await {
                        Ok(data) => break data,
                        Err(e) => {println!("{:?}", e)},
                    }
                }
            };
            let mut market_provider = CoingeckoMarketDataProvider::new(self.proxy_configs.clone()).await?;
            let _ = market_provider.cache_tokens_md(
                tokens_response.tokens.values().map(|t| t.address.clone()).collect::<HashSet<Address>>()
            ).await;

            let mut tokens_response_iter = tokens_response.tokens.iter();
            let mut requests_queue = FuturesUnordered::new();

            // TODO Rewrite
            // if let Some(md_provider) = market_data_provider.as_mut() {
            //     let t = md_provider.cache_tokens_md(tokens_response.tokens.values().map(|t| t.address.clone()).collect::<HashSet<Address>>());
            //     t.await;
            // };

            for _ in 1..5 {
                if let Some((_, token)) = tokens_response_iter.next() {
                    requests_queue.push(process_token(token.clone(), chain, filter, Some(&market_provider)));
                };
            };

            // let mut i = 0;
            while let Some(result) = requests_queue.next().await {
                if let Some(token) = result {
                    result_tokens.insert(token);
                };
                let _ = stdout().flush();
                if let Some((_, token)) = tokens_response_iter.next() {
                    requests_queue.push(process_token(token.clone(), chain, filter, Some(&market_provider)));
                };
                // i += 1;
                // print!("\rProcessing tokens: {i}/{}", tokens_response.tokens.len());
            }
        }
        println!();

        Ok(result_tokens)
    }
}

async fn process_token<'a, 'b>(
    token: OIRToken,
    chain: &'a Chain,
    filter: Option<&'a HashMap<String, Filter>>,
    market_data_provider: Option<&'b dyn MarketDataProvider>,
) -> Option<Token<'a>> {
    let market_data = match market_data_provider {
        Some(provider) => match provider.get_by_address(token.address.clone()).await {
            Ok(response) => match response {
                Some(data) => Some(data),
                None => {println!("{}: Not Found", token.symbol); None}
            },
            Err(e) => {println!("\nError in {}: {e}", token.symbol); None}
        },
        None => None
    };
    let token_candidate = Token {
        ticker: match &market_data {
            Some(data) => data.ticker.clone().to_uppercase(),
            None => token.symbol.clone(),

        },
        address: Some(token.address),
        name: Some(token.name.clone()),
        chain: &chain,
        decimals: Some(token.decimals),
        price_usd: match &market_data {
            Some(data) => Some(data.price_usd.clone()),
            None => None
        },
        volume_24h_usd: match &market_data {
            Some(data) => Some(data.volume_24.clone()),
            None => None
        },
        market_provider_id: match &market_data {
            Some(data) => Some(data.token_id.clone()),
            None => None
        },
    };
    match filter {
        Some(f) => {
            if token_candidate.match_filter(f) {
                return Some(token_candidate);
            };
            return None
        },
        _ => {
            return Some(token_candidate);
        }
    };
}

async fn get_quote<'a, 'b>(
    pair: &'a (&'a Token<'a>, &'a Token<'a>),
    client: &'b Client,
) -> Option<(&'a Token<'a>, &'a Token<'a>)> {

    let (from_token, to_token) = pair;

    let tokens_response = client.get(format!("https://api.1inch.exchange/v5.0/{}/quote", from_token.chain.id))
        .query(&[
            ("fromTokenAddress", &format!("{:#x}", from_token.address.unwrap())),
            ("toTokenAddress", &format!("{:#x}", to_token.address.unwrap())),
            (
                "amount",
                &(
                    format!(
                        "{}E{}",
                        from_token.price_usd.as_ref().unwrap_or(&String::from("1")),
                        from_token.decimals.unwrap()
                    ).parse::<f64>().unwrap_or(1.) * 100.
                ).to_string() // 100$ in tokens
            )
        ]).send().await;

    if let Ok(response) = &tokens_response {
        if response.status().is_success() {
            return Some((from_token, to_token))
            // println!("Success");
        }
        // println!("{:#?}", tokens_response
        //     .json::<OIRQuote>()
        //     .await?);
    }
    // println!("{:#?}", tokens_response.unwrap().status());
    return None;
    // write!(&mut stdout(), "{i}/{}, Successful: {}", token_pairs.len(), available_pairs.len());
}
