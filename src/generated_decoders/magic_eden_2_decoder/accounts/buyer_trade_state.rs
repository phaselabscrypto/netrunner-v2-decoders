
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xc8a499bb763cc833")] 
pub struct BuyerTradeState { 
        pub auction_house_key: solana_sdk::pubkey::Pubkey, 
        pub buyer: solana_sdk::pubkey::Pubkey, 
        pub buyer_referral: solana_sdk::pubkey::Pubkey, 
        pub buyer_price: u64, 
        pub token_mint: solana_sdk::pubkey::Pubkey, 
        pub token_size: u64, 
        pub bump: u8, 
        pub expiry: i64, 
}