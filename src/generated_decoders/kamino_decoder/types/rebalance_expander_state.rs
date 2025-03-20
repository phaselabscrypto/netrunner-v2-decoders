

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RebalanceExpanderState {
    pub initial_pool_price: u128,
    pub expansion_count: u16,
}
