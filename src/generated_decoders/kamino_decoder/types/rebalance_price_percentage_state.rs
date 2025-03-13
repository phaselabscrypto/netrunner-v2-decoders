use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RebalancePricePercentageState {
    pub last_rebalance_lower_pool_price: u128,
    pub last_rebalance_upper_pool_price: u128,
}
