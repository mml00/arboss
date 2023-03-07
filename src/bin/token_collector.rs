use std::collections::HashMap;
use screener::{
    exchange::{
        TExchangeProvider,
        providers,
    },
    core::{
        chain::get_available_chains,
        token::get_all_tokens,
    },
    config::{
        get_config,
        process_config_filters,
    },
    orm::{
        connection::get_connection,
        utils::update_tokens,
    },
    // market_data::CoingeckoMarketDataProvider,
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    // println!("{:#?}", config);

    let mut db = get_connection(config.database.clone()).await?;

    let chains = get_available_chains(&config.chains).await?;

    let token_filter = process_config_filters(String::from("tokens"), config.filters.clone());
    // let mut market_provider = CoingeckoMarketDataProvider::new(config.proxies.clone()).await?;

    // providers
    let mut providers: HashMap<&str, TExchangeProvider> = HashMap::new();
    let oneinch = providers::OneInchProvider::new(config.proxies.clone());
    providers.insert("1Inch", &oneinch);

    println!("Loading available tokens...");
    let tokens = get_all_tokens(
        &providers,
        &chains,
        Some(&token_filter),
        // &Some(&mut market_provider),
        &None,
    ).await?;

    println!("Tokens found: {}", &tokens.len());

    let t: Vec<_> = tokens.iter().collect();
    let _ = update_tokens(&mut db, &t).await;

    println!("Tokens updated successfully");

    Ok(())
}
