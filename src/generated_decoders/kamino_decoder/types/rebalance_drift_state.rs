
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RebalanceDriftState {
    pub step: RebalanceDriftStep,
    pub last_drift_timestamp: u64,
    pub last_mid_tick: i32,
}
