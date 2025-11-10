use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PricingParams {
    pub trade_spread_min: u64,
    pub trade_spread_max: u64,
    pub swap_spread: u64,
    pub min_init_leverage: u32,
    pub min_init_degen_leverage: u32,
    pub max_init_leverage: u32,
    pub max_init_degen_leverage: u32,
    pub max_leverage: u32,
    pub max_degen_leverage: u32,
    pub min_collateral_usd: u32,
    pub min_degen_collateral_usd: u32,
    pub delay_seconds: i64,
    pub max_utilization: u32,
    pub degen_position_factor: u16,
    pub degen_exposure_factor: u16,
    pub max_position_locked_usd: u64,
    pub max_exposure_usd: u64,
}
