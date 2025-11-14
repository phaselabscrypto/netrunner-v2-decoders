use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct GetAddCollateralQuoteData {
    pub amount_in: u64,
    pub collateral_delta: u64,
    pub final_collateral_amount: u64,
    pub final_collateral_usd: u64,
    pub final_size_amount: u64,
    pub final_size_usd: u64,
    pub final_avg_entry_price: OraclePrice,
    pub passes_min_collateral_check: bool,
    pub passes_leverage_check: bool,
    pub swap_required: bool,
    pub receiving_custody_id: u64,
    pub swap_price: OraclePrice,
    pub swap_fee_amount: u64,
    pub swap_fee_usd: u64,
    pub swap_possible: bool,
    pub increase_size_entry_price: OraclePrice,
    pub increase_size_fee_usd: u64,
    pub increase_size_fee_amount: u64,
}
