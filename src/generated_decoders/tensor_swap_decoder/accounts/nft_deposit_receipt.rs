
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xceff84fe434e3e60")] 
pub struct NftDepositReceipt { 
        pub bump: u8, 
        pub nft_authority: solana_sdk::pubkey::Pubkey, 
        pub nft_mint: solana_sdk::pubkey::Pubkey, 
        pub nft_escrow: solana_sdk::pubkey::Pubkey, 
}