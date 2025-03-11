
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9a3a941865c8f37f")] 
pub struct FreezePda { 
        pub candy_machine: solana_sdk::pubkey::Pubkey, 
        pub allow_thaw: bool, 
        pub frozen_count: u64, 
        pub mint_start: Option<i64>, 
        pub freeze_time: i64, 
        pub freeze_fee: u64, 
}