use web3::types::Address;
use serde::{Deserialize, Serialize};
use rbatis::{
    crud,
    impl_select,
};
use rbdc::{
    decimal::Decimal,
    datetime::FastDateTime
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenModel {
    pub id: Option<u64>,
    pub ticker: String,
    pub chain: String,
    pub decimals: Option<u8>,
    pub name: Option<String>,
    pub address: Option<Address>,
    pub price_usd: Option<Decimal>,
    pub volume_24h_usd: Option<Decimal>,
    pub market_provider_id: Option<String>,
    pub last_updated_at: FastDateTime,
    pub is_active: bool,
}

crud!(TokenModel {}, "token");

impl_select!(TokenModel {select_all_active(table_name: &str) => "`where is_active = true`"});
impl_select!(TokenModel {select_token(table_name: &str, ticker: String, chain: String) => "`where ticker = '${ticker}' and chain = '${chain}'`"});
impl_select!(TokenModel {select_all_active_with_market_data(table_name: &str) => "`where is_active = true and market_provider_id is not null`"});
