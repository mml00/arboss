use super::{
    MarketDataProvider,
    provider::TokenMarketData
};
use std::iter::FromIterator;
use web3::types::Address;
use serde::Deserialize;
use tokio::time::{sleep, Duration};
use std::{
    fmt::Debug,
    collections::{
        HashMap,
        HashSet,
    },
    str::FromStr,
};
use reqwest::{
    Client,
    Proxy
};

use crate::{
    config::{
        CProxy,
    },
};
use rand::random;


#[derive(Debug, Clone, Deserialize)]
struct CGRToken {
    pub id: String,
    pub symbol: String,
    // pub name: String,
    pub platforms: Option<HashMap<String, Option<String>>>,
}

type CGRAvailableTokens = Vec<CGRToken>;

#[derive(Debug, Clone, Deserialize)]
struct CGRMarketData {
    pub usd: f32,
    pub usd_24h_vol: f32,
}

type CGRTokenMarketData = Option<HashMap<String, Option<CGRMarketData>>>;

#[derive(Debug, Clone)]
pub struct CoingeckoMarketDataProvider {
    available_tokens_cache: CGRAvailableTokens,
    reqwest_clients: Vec<Client>,
    cache_md: Option<HashMap<Address, TokenMarketData>>
}


impl CoingeckoMarketDataProvider {
    pub async fn new(
        proxy_configs: Vec<CProxy>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut clients: Vec<Client> = Vec::new();
        for proxy in proxy_configs {
            clients.push(
                Client::builder().proxy(
                    Proxy::https(&proxy.url)
                        .unwrap()
                        .basic_auth(&proxy.user, &proxy.password)
                ).build().unwrap()
            )
        }
        let available_tokens_response = loop {
            let resp = clients[(random::<f32>() * clients.len() as f32).floor() as usize]
                .get("https://api.coingecko.com/api/v3/coins/list?include_platform=true")
                .send()
                .await?;
            if resp.status().is_success() {
                match resp.json::<CGRAvailableTokens>().await {
                    Ok(data) => break data,
                    Err(e) => {
                        println!("{e}");
                    },
                }
            } else {
                println!("Error: {:#?}", resp.status());
                sleep(Duration::from_secs(10)).await;
            }
        };
        Ok(
            Self {
                available_tokens_cache: available_tokens_response,
                reqwest_clients: clients,
                cache_md: None
            }
        )
    }
}

#[async_trait::async_trait]
impl MarketDataProvider for CoingeckoMarketDataProvider {
    //TODO Rewrite
    async fn cache_tokens_md(&mut self, addresses: HashSet<Address>) -> Result<(), Box<dyn std::error::Error>> {
        let mut id_address_pair = HashMap::new();

        for market_data in self.available_tokens_cache.iter() {
            if let Some(platforms) = &market_data.platforms {
                let token_addresses = platforms
                    .values()
                    .map(|a| match Address::from_str(a.as_ref().unwrap_or(&String::from("0x0000000000000000000000000000000000000000")).as_str()) {
                        Ok(address) => address,
                        Err(_) => Address::from_str("0x0000000000000000000000000000000000000000").unwrap(),
                    })
                    .collect::<HashSet<Address>>();

                let address_candidates = addresses.intersection(&token_addresses).collect::<Vec<&Address>>();

                if address_candidates.len() >= 1 {
                    id_address_pair.insert(market_data.id.clone(), (market_data.symbol.clone(), address_candidates[0].clone()));
                }
            }
        }

        id_address_pair.remove("wrapped-ethw".into()); // ETH duplicate

        // id_address_pair.insert("weth".into(), Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap()); // ETH
        // id_address_pair.insert("weth".into(), Address::from_str("0x7ceb23fd6bc0add59e62ac25578270cff1b9f619").unwrap()); // ETH polygon
        // id_address_pair.insert("matic-network".into(), Address::from_str("0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee").unwrap()); // MATIC
        // id_address_pair.insert("matic-network".into(), Address::from_str("0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0").unwrap()); // MATIC
        // id_address_pair.insert("wrapped-bitcoin".into(), Address::from_str("0x0000000000000000000000000000000000001010").unwrap()); // MATIC

        println!("Market provider matches: {:#?}", id_address_pair.len());


        let mut market_data_response: Vec<(String, Option<CGRMarketData>)> = vec![];
        for chunk in id_address_pair.keys().map(|a| a.clone()).collect::<Vec<String>>().chunks(200).collect::<Vec<_>>() {
            loop {
                let resp = self.reqwest_clients[(random::<f32>() * self.reqwest_clients.len() as f32).floor() as usize]
                    .get(format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd&include_24hr_vol=true", chunk.join(",")))
                    .send()
                    .await?;
                if resp.status().is_success() {
                    if let Ok(Some(data)) = resp.json::<CGRTokenMarketData>().await {
                        market_data_response.extend(Vec::from_iter(data.into_iter()));
                        break
                    }
                } else {
                    println!("Error: {:#?}", resp.status());
                    sleep(Duration::from_secs(5)).await;
                }
            };
        }
        let mut cache = HashMap::new();
        for (id, market_data) in market_data_response.iter() {
            let md = market_data.as_ref().unwrap();
            let (ticker, address) = id_address_pair.get(id).unwrap().clone();
            cache.insert(address, TokenMarketData {
                ticker: ticker,
                token_id: id.clone(),
                price_usd: format!("{}", md.usd),
                volume_24: format!("{}", md.usd_24h_vol),
            });
        }
        self.cache_md = Some(cache);
        Ok(())
    }

    async fn get_by_address(&self, address: Address) -> Result<Option<TokenMarketData>, Box<dyn std::error::Error>> {
        Ok(self.cache_md.as_ref().unwrap().get(&address).cloned())
    }
}
