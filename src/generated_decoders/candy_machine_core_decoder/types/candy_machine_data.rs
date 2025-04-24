use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CandyMachineData {
    pub items_available: u64,
    pub max_supply: u64,
    pub is_mutable: bool,
    pub config_line_settings: Option<ConfigLineSettings>,
    pub hidden_settings: Option<HiddenSettings>,
}
