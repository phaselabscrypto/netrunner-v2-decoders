use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CloseAndSwapParams {
    pub price_with_slippage: OraclePrice,
    pub min_swap_amount_out: u64,
    pub privilege: Privilege,
}
