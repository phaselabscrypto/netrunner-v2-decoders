use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitParams {
    pub min_signatures: u8,
    pub permissions: Permissions,
    pub voltage_multiplier: VoltageMultiplier,
    pub trading_discount: [u64; 6],
    pub referral_rebate: [u64; 6],
    pub default_rebate: u64,
}
