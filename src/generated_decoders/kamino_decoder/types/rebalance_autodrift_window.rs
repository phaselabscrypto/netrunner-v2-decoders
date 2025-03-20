
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RebalanceAutodriftWindow {
    pub staking_rate_a: Option<Price>,
    pub staking_rate_b: Option<Price>,
    pub epoch: u64,
    pub theoretical_tick: i32,
    pub strat_mid_tick: i32,
}
