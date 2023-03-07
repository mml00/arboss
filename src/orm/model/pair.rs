use web3::types::Address;
use serde::{Deserialize, Serialize};
use rbdc::datetime::FastDateTime;
use rbatis::{
    crud,
    impl_select,
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PairModel {
    pub id: Option<u64>,
    pub from_token_id: u64,
    pub to_token_id: u64,
    pub providers: String,
    pub pool_address: Option<Address>,
    pub fee_percent: Option<String>,
    pub last_updated_at: FastDateTime,
    pub is_active: bool,
}

crud!(PairModel {}, "pair");

impl_select!(PairModel {select_pair(table_name: &str, from_token_id: u64, to_token_id: u64) =>
                         "`where is_active = true and from_token_id = ${from_token_id} and to_token_id = ${to_token_id}`"});
