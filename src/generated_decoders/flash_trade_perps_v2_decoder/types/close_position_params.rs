use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ClosePositionParams {
    pub price_with_slippage: OraclePrice,
    pub privilege: Privilege,
}
