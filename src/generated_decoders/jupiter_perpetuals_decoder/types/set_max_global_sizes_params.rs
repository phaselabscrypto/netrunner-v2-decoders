use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetMaxGlobalSizesParams {
    pub max_global_long_size: u64,
    pub max_global_short_size: u64,
    pub recovery_id: u8,
    #[serde(with = "BigArray")]
    pub signature: [u8; 64],
    pub reference_id: [u8; 16],
    pub timestamp: u64,
}
