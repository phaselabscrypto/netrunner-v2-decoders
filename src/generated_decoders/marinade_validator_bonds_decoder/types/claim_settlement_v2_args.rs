

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ClaimSettlementV2Args {
    pub proof: Vec<[u8; 32]>,
    pub tree_node_hash: [u8; 32],
    pub stake_account_staker: solana_sdk::pubkey::Pubkey,
    pub stake_account_withdrawer: solana_sdk::pubkey::Pubkey,
    pub claim: u64,
    pub index: u64,
}
