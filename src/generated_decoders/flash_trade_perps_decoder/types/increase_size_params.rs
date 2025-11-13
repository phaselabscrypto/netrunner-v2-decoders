use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct IncreaseSizeParams {
    pub price_with_slippage: OraclePrice,
    pub size_delta: u64,
    pub privilege: Privilege,
}
