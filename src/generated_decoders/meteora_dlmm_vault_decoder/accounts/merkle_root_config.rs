
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x6702ded94932bb27")] 
pub struct MerkleRootConfig { 
        pub root: [u8; 32], 
        pub vault: solana_sdk::pubkey::Pubkey, 
        pub version: u64, 
        pub padding: [u128; 4], 
}