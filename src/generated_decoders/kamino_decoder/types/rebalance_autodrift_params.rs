use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RebalanceAutodriftParams {
    pub init_drift_ticks_per_epoch: u32,
    pub ticks_below_mid: i32,
    pub ticks_above_mid: i32,
    pub frontrun_multiplier_bps: u16,
    pub staking_rate_a_source: StakingRateSource,
    pub staking_rate_b_source: StakingRateSource,
    pub init_drift_direction: DriftDirection,
}
