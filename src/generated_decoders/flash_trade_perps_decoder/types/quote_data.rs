use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct QuoteData {
    pub amount_in: u64,
    pub collateral_amount: u64,
    pub collateral_usd: u64,
    pub size_amount: u64,
    pub size_usd: u64,
    pub entry_price: OraclePrice,
    pub total_fee_amount: u64,
    pub total_fee_usd: u64,
    pub entry_fee_amount: u64,
    pub entry_fee_usd: u64,
    pub volatility_fee_amount: u64,
    pub volatility_fee_usd: u64,
    pub swap_required: bool,
    pub swap_fee_amount: u64,
    pub swap_fee_usd: u64,
    pub liquidation_price: OraclePrice,
}
