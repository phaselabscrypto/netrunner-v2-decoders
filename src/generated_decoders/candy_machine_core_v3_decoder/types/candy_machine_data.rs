
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CandyMachineData {
    pub items_available: u64,
    pub symbol: String,
    pub seller_fee_basis_points: u16,
    pub max_supply: u64,
    pub is_mutable: bool,
    pub creators: Vec<Creator>,
    pub config_line_settings: Option<ConfigLineSettings>,
    pub hidden_settings: Option<HiddenSettings>,
}
