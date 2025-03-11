
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x0e72d48c18861f18")] 
pub struct SingleListing { 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub nft_mint: solana_sdk::pubkey::Pubkey, 
        pub price: u64, 
        pub bump: [u8; 1], 
        pub reserved: [u8; 64], 
}