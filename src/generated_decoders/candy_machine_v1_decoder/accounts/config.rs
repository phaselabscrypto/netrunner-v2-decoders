
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9b0caae01efacc82")] 
pub struct Config { 
        pub authority: solana_sdk::pubkey::Pubkey, 
        pub data: ConfigData, 
}