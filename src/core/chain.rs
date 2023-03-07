use crate::config::CChain;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Chain {
    pub id: u32,
    pub name: String,
    pub rpc_url: String,
}

pub async fn get_available_chains(
    chains_config: &Vec<CChain>
) -> Result<Vec<Chain>, Box<dyn std::error::Error>> {

    let mut chains: Vec<Chain> = vec![];

    for chain_config in chains_config.iter() {
        chains.push(Chain {
            id: chain_config.id,
            name: String::from(&chain_config.name),
            rpc_url: String::from(&chain_config.rpc_url),
        });
    }

    Ok(chains)
}
