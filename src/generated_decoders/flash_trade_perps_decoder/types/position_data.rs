use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PositionData {
    pub collateral_usd: u64,
    pub profit_usd: u64,
    pub loss_usd: u64,
    pub fee_usd: u64,
    pub leverage: u64,
    pub liquidation_price: OraclePrice,
}
