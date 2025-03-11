
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xa40e5c647b39eacc")] 
pub struct SellerTradeStateV2 { 
        pub auction_house_key: solana_sdk::pubkey::Pubkey, 
        pub seller: solana_sdk::pubkey::Pubkey, 
        pub seller_referral: solana_sdk::pubkey::Pubkey, 
        pub buyer_price: u64, 
        pub token_mint: solana_sdk::pubkey::Pubkey, 
        pub token_account: solana_sdk::pubkey::Pubkey, 
        pub token_size: u64, 
        pub bump: u8, 
        pub expiry: i64, 
        pub payment_mint: solana_sdk::pubkey::Pubkey, 
}