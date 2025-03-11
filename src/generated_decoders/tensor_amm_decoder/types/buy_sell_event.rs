

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct BuySellEvent {
    pub current_price: u64,
    pub taker_fee: u64,
    pub mm_fee: u64,
    pub creators_fee: u64,
}
