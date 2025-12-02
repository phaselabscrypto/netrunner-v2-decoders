use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetMarketConfigParams {
    pub max_payoff_bps: u64,
    pub permissions: MarketPermissions,
    pub correlation: bool,
}
