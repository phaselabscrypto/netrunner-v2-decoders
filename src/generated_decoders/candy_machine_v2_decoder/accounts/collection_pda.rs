
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xcb80777dea59e89d")] 
pub struct CollectionPda { 
        pub mint: solana_sdk::pubkey::Pubkey, 
        pub candy_machine: solana_sdk::pubkey::Pubkey, 
}