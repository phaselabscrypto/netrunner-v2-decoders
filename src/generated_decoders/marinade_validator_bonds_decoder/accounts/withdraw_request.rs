
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xbaefaebfbd0d2fc4")] 
pub struct WithdrawRequest { 
        pub vote_account: solana_sdk::pubkey::Pubkey, 
        pub bond: solana_sdk::pubkey::Pubkey, 
        pub epoch: u64, 
        pub requested_amount: u64, 
        pub withdrawn_amount: u64, 
        pub bump: u8, 
        pub reserved: [u8; 93], 
}