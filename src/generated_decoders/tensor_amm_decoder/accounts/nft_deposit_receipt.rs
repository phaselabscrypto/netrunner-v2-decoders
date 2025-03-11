
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xceff84fe434e3e60")] 
pub struct NftDepositReceipt { 
        pub bump: u8, 
        pub mint: solana_sdk::pubkey::Pubkey, 
        pub pool: solana_sdk::pubkey::Pubkey, 
}