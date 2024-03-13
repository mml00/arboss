use std::collections::HashMap;
use screener::{
    config::get_config,
    core::chain::get_available_chains,
    exchange::{
        TExchangeProvider,
        providers,
    },
    orm::{
        utils::{
            get_tokens,
            update_pairs,
        },
        connection::get_connection
    },
    utils::{
        get_possible_pairs_enumerations,
        validate_pairs,
    },
};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let mut db = get_connection(config.database.clone()).await?;

    let chains = get_available_chains(&config.chains).await?;

    let tokens = get_tokens(&mut db, &chains).await?;
    println!("Tokens: {:?}", tokens.len());

    let possible_pairs_enumerations = get_possible_pairs_enumerations(&tokens, config.crosschain);
    println!("Possible pairs found: {}", &possible_pairs_enumerations.len());

    // providers
    let mut providers: HashMap<&str, TExchangeProvider> = HashMap::new();
    let oneinch = providers::OneInchProvider::new(config.proxies.clone());
    providers.insert("1Inch", &oneinch);

    use std::time::{SystemTime};
    loop {
        let start = SystemTime::now();
        // possible_pairs_enumerations.sort_by_key(|p| (p.0.ticker.clone(), p.1.ticker.clone()));
        // let p = possible_pairs_enumerations[0..500].to_vec();
        // let valid_pairs = validate_pairs(&p, &providers, crosschain).await?;
        let valid_pairs = validate_pairs(&possible_pairs_enumerations, &providers, crosschain).await?;
        println!("Valid pairs found: {} ({}s)", &valid_pairs.len(), SystemTime::now().duration_since(start).unwrap_or_default().as_secs());

        update_pairs(&mut db, &valid_pairs).await?;
    }

    // Ok(())
}
