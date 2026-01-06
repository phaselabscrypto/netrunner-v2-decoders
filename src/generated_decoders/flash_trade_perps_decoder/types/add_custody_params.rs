use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct AddCustodyParams {
    pub is_stable: bool,
    pub depeg_adjustment: bool,
    pub is_virtual: bool,
    pub token22: bool,
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub fees: Fees,
    pub borrow_rate: BorrowRateParams,
    pub ratios: Vec<TokenRatios>,
    pub reward_threshold: u64,
    pub min_reserve_usd: u64,
    pub limit_price_buffer_bps: u64,
}
