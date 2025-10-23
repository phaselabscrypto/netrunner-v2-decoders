use super::*;
use carbon_core::{borsh, CarbonDeserialize};
use serde_big_array::BigArray;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TickArrayState {
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub start_tick_index: i32,
    #[serde(with = "BigArray")]
    pub ticks: [TickState; 60],
    pub initialized_tick_count: u8,
    pub recent_epoch: u64,
    #[serde(with = "BigArray")]
    pub padding: [u8; 107],
}
