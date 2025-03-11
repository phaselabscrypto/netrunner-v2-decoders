
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x85dcadd5b3d32bee")] 
pub struct MarginAccount { 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub name: [u8; 32], 
        pub nr: u16, 
        pub bump: [u8; 1], 
        pub pools_attached: u32, 
        pub reserved: [u8; 64], 
}