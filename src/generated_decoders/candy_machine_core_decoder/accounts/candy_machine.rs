
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x33adb17119f16dbd")] 
pub struct CandyMachine { 
        pub authority: solana_sdk::pubkey::Pubkey, 
        pub mint_authority: solana_sdk::pubkey::Pubkey, 
        pub collection_mint: solana_sdk::pubkey::Pubkey, 
        pub items_redeemed: u64, 
        pub data: CandyMachineData, 
}