
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9312633af908c4dd")] 
pub struct AssetDepositReceipt { 
        pub bump: u8, 
        pub asset: solana_sdk::pubkey::Pubkey, 
        pub pool: solana_sdk::pubkey::Pubkey, 
}