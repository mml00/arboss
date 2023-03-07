use crate::{
    Pair,
    exchange::TExchangeProvider
};


#[derive(Debug)]
pub struct Swap<'a> {
    pub pair: &'a Pair<'a>,
    pub amount: u128,
    pub price: String,
    pub provider: &'a TExchangeProvider<'a>,
}
