use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RebalanceDriftParams {
    pub start_mid_tick: i32,
    pub ticks_below_mid: i32,
    pub ticks_above_mid: i32,
    pub seconds_per_tick: u64,
    pub direction: DriftDirection,
}
