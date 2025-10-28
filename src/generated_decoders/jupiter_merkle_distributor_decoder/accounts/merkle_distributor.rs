use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x4d778b4654f70c1a")]
pub struct MerkleDistributor {
    pub root: [u8; 32],
    pub mint: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub clawback_receiver: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub version: u64,
    pub max_total_claim: u64,
    pub max_num_nodes: u64,
    pub total_amount_claimed: u64,
    pub num_nodes_claimed: u64,
    pub start_ts: i64,
    pub end_ts: i64,
    pub clawback_start_ts: i64,
    pub activation_point: u64,
    pub activation_type: u8,
    pub claim_type: u8,
    pub bump: u8,
    pub clawed_back: u8,
    pub closable: u8,
    pub padding0: [u8; 3],
    pub airdrop_bonus: AirdropBonus,
    pub padding2: [u128; 5],
}
