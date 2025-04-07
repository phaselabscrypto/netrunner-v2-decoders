
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9b0caae01efacc82")] 
pub struct Config { 
        pub admin_authority: solana_sdk::pubkey::Pubkey, 
        pub operator_authority: solana_sdk::pubkey::Pubkey, 
        pub epochs_to_claim_settlement: u64, 
        pub withdraw_lockup_epochs: u64, 
        pub minimum_stake_lamports: u64, 
        pub bonds_withdrawer_authority_bump: u8, 
        pub pause_authority: solana_sdk::pubkey::Pubkey, 
        pub paused: bool, 
        pub slots_to_start_settlement_claiming: u64, 
        pub min_bond_max_stake_wanted: u64, 
        pub reserved: [u8; 463], 
}