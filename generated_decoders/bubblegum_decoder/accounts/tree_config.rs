
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x7af5aff8ab2200cf")] 
pub struct TreeConfig { 
        pub tree_creator: solana_sdk::pubkey::Pubkey, 
        pub tree_delegate: solana_sdk::pubkey::Pubkey, 
        pub total_mint_capacity: u64, 
        pub num_minted: u64, 
        pub is_public: bool, 
        pub is_decompressible: DecompressibleState, 
}