

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1de2ad6f6f69da7667")]
pub struct CloseSettlementEvent{
    pub bond: solana_sdk::pubkey::Pubkey,
    pub settlement: solana_sdk::pubkey::Pubkey,
    pub merkle_root: [u8; 32],
    pub max_total_claim: u64,
    pub max_merkle_nodes: u64,
    pub lamports_funded: u64,
    pub lamports_claimed: u64,
    pub merkle_nodes_claimed: u64,
    pub split_rent_collector: Option<solana_sdk::pubkey::Pubkey>,
    pub split_rent_refund: Option<solana_sdk::pubkey::Pubkey>,
    pub rent_collector: solana_sdk::pubkey::Pubkey,
    pub expiration_epoch: u64,
    pub current_epoch: u64,
}
