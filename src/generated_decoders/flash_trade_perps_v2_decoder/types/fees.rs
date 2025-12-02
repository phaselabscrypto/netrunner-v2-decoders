use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Fees {
    pub mode: FeesMode,
    pub swap_in: RatioFees,
    pub swap_out: RatioFees,
    pub stable_swap_in: RatioFees,
    pub stable_swap_out: RatioFees,
    pub add_liquidity: RatioFees,
    pub remove_liquidity: RatioFees,
    pub open_position: u64,
    pub close_position: u64,
    pub volatility: u64,
}
