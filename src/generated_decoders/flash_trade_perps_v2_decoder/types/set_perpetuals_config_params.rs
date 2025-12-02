use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetPerpetualsConfigParams {
    pub allow_ungated_trading: bool,
    pub trading_discount: [u64; 6],
    pub referral_rebate: [u64; 6],
    pub default_rebate: u64,
    pub voltage_multiplier: VoltageMultiplier,
    pub trade_limit: u8,
    pub trigger_order_limit: u8,
    pub rebate_limit_usd: u32,
}
