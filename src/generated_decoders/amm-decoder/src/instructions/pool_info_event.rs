

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dcf145761fbd4ea2d")]
pub struct PoolInfoEvent{
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub virtual_price: f64,
    pub current_timestamp: u64,
}
