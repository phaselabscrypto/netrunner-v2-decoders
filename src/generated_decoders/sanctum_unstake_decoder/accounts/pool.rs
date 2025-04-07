
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf19a6d0411b16dbc")] 
pub struct Pool { 
        pub fee_authority: solana_sdk::pubkey::Pubkey, 
        pub lp_mint: solana_sdk::pubkey::Pubkey, 
        pub incoming_stake: u64, 
}