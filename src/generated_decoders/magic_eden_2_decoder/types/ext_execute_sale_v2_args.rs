

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ExtExecuteSaleV2Args {
    pub price: u64,
    pub maker_fee_bp: i16,
    pub taker_fee_bp: u16,
}
