use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RebalanceExpanderState {
    pub initial_pool_price: u128,
    pub expansion_count: u16,
}
