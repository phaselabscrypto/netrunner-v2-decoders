use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapAndOpenParams {
    pub price_with_slippage: OraclePrice,
    pub amount_in: u64,
    pub size_amount: u64,
    pub privilege: Privilege,
}
