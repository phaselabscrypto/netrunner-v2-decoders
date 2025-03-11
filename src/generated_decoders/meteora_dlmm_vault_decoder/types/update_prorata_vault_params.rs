

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct UpdateProrataVaultParams {
    pub max_buying_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
}
