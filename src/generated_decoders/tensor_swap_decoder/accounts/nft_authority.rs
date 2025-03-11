
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xc27fdb10db12fa0c")] 
pub struct NftAuthority { 
        pub random_seed: [u8; 32], 
        pub bump: [u8; 1], 
        pub pool: solana_sdk::pubkey::Pubkey, 
}