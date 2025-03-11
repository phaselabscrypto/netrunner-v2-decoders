
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x01ee48898a15fef9")] 
pub struct SellerTradeState { 
        pub auction_house_key: solana_sdk::pubkey::Pubkey, 
        pub seller: solana_sdk::pubkey::Pubkey, 
        pub seller_referral: solana_sdk::pubkey::Pubkey, 
        pub buyer_price: u64, 
        pub token_mint: solana_sdk::pubkey::Pubkey, 
        pub token_account: solana_sdk::pubkey::Pubkey, 
        pub token_size: u64, 
        pub bump: u8, 
        pub expiry: i64, 
}