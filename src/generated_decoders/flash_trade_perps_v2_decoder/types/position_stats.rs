use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PositionStats {
    pub open_positions: u64,
    pub update_time: i64,
    pub average_entry_price: OraclePrice,
    pub size_amount: u64,
    pub size_usd: u64,
    pub locked_amount: u64,
    pub locked_usd: u64,
    pub collateral_amount: u64,
    pub collateral_liability_usd: u64,
    pub unsettled_fee_usd: u64,
    pub cumulative_lock_fee_snapshot: u128,
    pub size_decimals: u8,
    pub locked_decimals: u8,
    pub collateral_decimals: u8,
}
