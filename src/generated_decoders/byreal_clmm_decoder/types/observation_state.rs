use super::*;
use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ObservationState {
    pub initialized: bool,
    pub recent_epoch: u64,
    pub observation_index: u16,
    pub pool_id: solana_sdk::pubkey::Pubkey,
    #[serde(with = "BigArray")]
    pub observations: [Observation; 100],
    pub padding: [u64; 4],
}
