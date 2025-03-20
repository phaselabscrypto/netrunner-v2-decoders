
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct RebalanceAutodriftState {
    pub last_window: RebalanceAutodriftWindow,
    pub current_window: RebalanceAutodriftWindow,
    pub step: RebalanceAutodriftStep,
}
