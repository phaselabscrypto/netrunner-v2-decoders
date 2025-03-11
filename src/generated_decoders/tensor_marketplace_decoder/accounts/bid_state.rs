
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x9bc50561bd3c08b7")] 
pub struct BidState { 
        pub version: u8, 
        pub bump: [u8; 1], 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub bid_id: solana_sdk::pubkey::Pubkey, 
        pub target: Target, 
        pub target_id: solana_sdk::pubkey::Pubkey, 
        pub field: Option<Field>, 
        pub field_id: Option<solana_sdk::pubkey::Pubkey>, 
        pub quantity: u32, 
        pub filled_quantity: u32, 
        pub amount: u64, 
        pub currency: Option<solana_sdk::pubkey::Pubkey>, 
        pub expiry: i64, 
        pub private_taker: Option<solana_sdk::pubkey::Pubkey>, 
        pub maker_broker: Option<solana_sdk::pubkey::Pubkey>, 
        pub margin: Option<solana_sdk::pubkey::Pubkey>, 
        pub updated_at: i64, 
        pub cosigner: solana_sdk::pubkey::Pubkey, 
        pub rent_payer: solana_sdk::pubkey::Pubkey, 
        pub reserved: [u8; 8], 
        pub reserved1: [u8; 16], 
        pub reserved2: [u8; 32], 
}