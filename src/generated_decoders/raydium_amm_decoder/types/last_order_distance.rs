

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LastOrderDistance {
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
}
