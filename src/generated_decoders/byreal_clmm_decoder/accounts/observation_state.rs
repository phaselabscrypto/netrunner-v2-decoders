use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x7aaec5358109a584")]
pub struct ObservationState {
    pub initialized: bool,
    pub recent_epoch: u64,
    pub observation_index: u16,
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub observations: [Observation; 100],
    pub padding: [u64; 4],
}
