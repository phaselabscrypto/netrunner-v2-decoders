

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbbc32e817453e7f1")]
pub struct InitSettlementEvent{
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub vote_account: solana_sdk::pubkey::Pubkey,
    pub staker_authority: solana_sdk::pubkey::Pubkey,
    pub merkle_root: [u8; 32],
    pub max_total_claim: u64,
    pub max_merkle_nodes: u64,
    pub epoch_created_for: u64,
    pub slot_created_at: u64,
    pub rent_collector: solana_sdk::pubkey::Pubkey,
}
