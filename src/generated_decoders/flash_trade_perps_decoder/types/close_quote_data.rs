use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CloseQuoteData {
    pub size_amount: u64,
    pub size_usd: u64,
    pub exit_price: OraclePrice,
    pub collateral_return: u64,
    pub fee_amount: u64,
    pub fee_usd: u64,
    pub pnl_amount: u64,
    pub pnl_usd: u64,
    pub is_profitable: bool,
    pub remaining_collateral: u64,
    pub remaining_size: u64,
}
