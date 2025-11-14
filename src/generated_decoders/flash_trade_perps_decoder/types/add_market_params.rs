use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddMarketParams {
    pub side: Side,
    pub correlation: bool,
    pub max_payoff_bps: u64,
    pub permissions: MarketPermissions,
}
