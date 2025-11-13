use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapAndOpenParams {
    pub amount_in: u64,
    pub min_collateral_amount_out: u64,
    pub price_with_slippage: OraclePrice,
    pub size_amount: u64,
    pub privilege: Privilege,
}
