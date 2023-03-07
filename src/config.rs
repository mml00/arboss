use serde::Deserialize;
use serde_json::from_reader;
use json_comments::StripComments;
use std::{
    fs::File,
    path::Path,
    collections::HashMap,
};

use crate::utils::filters::{
    Filter,
    FieldFilter,
    LogicalWrapper,
};


// Config types
#[derive(Debug, Deserialize)]
pub struct CChain {
    pub id: u32,
    pub name: String,
    pub rpc_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CProxy {
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum CFilterOptions {
    And,
    Or,
    Not,
    In,
    IsExact,
    Contains,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CFilter {
    pub op: CFilterOptions,
    pub filters: Option<Vec<CFilter>>,
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CTokenFilters {
    pub ticker: CFilter,
    // pub ticker: CFilter,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CFilters {
    pub tokens: CTokenFilters,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CDatabase {
    pub user: String,
    pub password: String,
    pub database: String,
    pub url: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database: CDatabase,
    pub chains: Vec<CChain>,
    pub filters: CFilters,
    pub proxies: Vec<CProxy>
}


pub fn get_config() -> Config {
    println!("Loading config from: {:#?}", Path::new("src/config.json").canonicalize().unwrap());

    let file = File::open(Path::new("src/config.json")).unwrap();

    from_reader(StripComments::new(file)).expect("JSON was not well-formatted")
}

pub fn process_config_filters(target: String, filters_config: CFilters) -> HashMap<String, Filter> {
    let mut filter = HashMap::new();
    match target.as_str() {
        "tokens" => {
            let tokens = filters_config.tokens;

            filter.insert(String::from("ticker"), build_filter(&tokens.ticker));

            fn build_filter(filter_node: &CFilter) -> Filter {
                match filter_node.op {
                    CFilterOptions::And => Filter::Operator(
                        Box::new(LogicalWrapper::And(filter_node.filters.clone().unwrap().iter().map(|f| build_filter(f)).collect::<Vec<Filter>>())),
                    ),
                    CFilterOptions::Or => Filter::Operator(
                        Box::new(LogicalWrapper::Or(filter_node.filters.clone().unwrap().iter().map(|f| build_filter(f)).collect::<Vec<Filter>>())),
                    ),
                    CFilterOptions::Not => Filter::Operator(
                        Box::new(LogicalWrapper::Not(filter_node.filters.clone().unwrap().iter().map(|f| build_filter(f)).collect::<Vec<Filter>>()[0].clone())),
                    ),
                    CFilterOptions::In => Filter::Filter(
                        FieldFilter::In(filter_node.value.clone().unwrap())
                    ),
                    CFilterOptions::IsExact => Filter::Filter(
                        FieldFilter::IsExact(filter_node.value.clone().unwrap()[0].clone())
                    ),
                    CFilterOptions::Contains => Filter::Filter(
                        FieldFilter::Contains(filter_node.value.clone().unwrap()[0].clone())
                    ),
                }
            }
        }
        _ => {}
    };
    filter
}
