
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x554071c6ea5e787b")] 
pub struct TipDistributionAccount { 
        pub validator_vote_account: solana_sdk::pubkey::Pubkey, 
        pub merkle_root_upload_authority: solana_sdk::pubkey::Pubkey, 
        pub merkle_root: Option<MerkleRoot>, 
        pub epoch_created_at: u64, 
        pub validator_commission_bps: u16, 
        pub expires_at: u64, 
        pub bump: u8, 
}