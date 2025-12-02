use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OpenPositionParams {
    pub price_with_slippage: OraclePrice,
    pub collateral_amount: u64,
    pub size_amount: u64,
    pub privilege: Privilege,
}
