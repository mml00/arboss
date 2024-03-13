use std::collections::HashMap;
use screener::{
    config::get_config,
    core::{
        chain::get_available_chains,
        swap::fetch_swaps,
    },
    exchange::{
        TExchangeProvider,
        providers,
    },
    orm::{
        utils::{
            get_tokens,
            get_valid_pairs,
            update_swaps,
        },
        connection::get_connection
    },
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let mut db = get_connection(config.database.clone()).await?;

    let chains = get_available_chains(&config.chains).await?;
    let tokens = get_tokens(&mut db, &chains).await?;
    println!("Tokens: {:?}", tokens.len());

    let valid_pairs = get_valid_pairs(&mut db, &tokens).await?;
    println!("Valid pairs: {:?}", valid_pairs.len());

    // providers
    let mut providers: HashMap<&str, TExchangeProvider> = HashMap::new();
    let oneinch = providers::OneInchProvider::new(config.proxies.clone());
    providers.insert("1Inch", &oneinch);

    let swaps = fetch_swaps(&valid_pairs, &providers, config.crosschain).await?;

    update_swaps(&mut db, &swaps).await;

    Ok(())
}
