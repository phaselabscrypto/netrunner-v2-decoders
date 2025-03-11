

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CoreSellArgs {
    pub price: u64,
    pub expiry: i64,
    pub compression_proof: Option<Vec<u8>>,
}
