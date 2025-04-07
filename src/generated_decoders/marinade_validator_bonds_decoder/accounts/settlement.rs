
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x370bdb21248828b6")] 
pub struct Settlement { 
        pub bond: solana_sdk::pubkey::Pubkey, 
        pub staker_authority: solana_sdk::pubkey::Pubkey, 
        pub merkle_root: [u8; 32], 
        pub max_total_claim: u64, 
        pub max_merkle_nodes: u64, 
        pub lamports_funded: u64, 
        pub lamports_claimed: u64, 
        pub merkle_nodes_claimed: u64, 
        pub epoch_created_for: u64, 
        pub slot_created_at: u64, 
        pub rent_collector: solana_sdk::pubkey::Pubkey, 
        pub split_rent_collector: Option<solana_sdk::pubkey::Pubkey>, 
        pub split_rent_amount: u64, 
        pub bumps: Bumps, 
        pub reserved: [u8; 90], 
}