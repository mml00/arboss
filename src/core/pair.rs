use web3::types::Address;
use crate::{
    Token,
};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair<'a> {
    pub from: &'a Token<'a>,
    pub to: &'a Token<'a>,
    pub providers: Option<Vec<&'a str>>,
    // Dex
    pub pool_address: Option<Address>,
    pub fee_percent: Option<String>,
}

impl<'a> Pair<'a> {
    pub fn get_providers_mut(&'a mut self) -> &mut Option<Vec<&'a str>> {
        &mut self.providers
    }
}
