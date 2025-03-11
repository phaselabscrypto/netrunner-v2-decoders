
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x2ccfc7b8706722b5")] 
pub struct CandyGuard { 
        pub base: solana_sdk::pubkey::Pubkey, 
        pub bump: u8, 
        pub authority: solana_sdk::pubkey::Pubkey, 
}