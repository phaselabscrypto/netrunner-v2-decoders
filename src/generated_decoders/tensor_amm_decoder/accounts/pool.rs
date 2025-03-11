
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf19a6d0411b16dbc")] 
pub struct Pool { 
        pub version: u8, 
        pub bump: [u8; 1], 
        pub pool_id: [u8; 32], 
        pub created_at: i64, 
        pub updated_at: i64, 
        pub expiry: i64, 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub whitelist: solana_sdk::pubkey::Pubkey, 
        pub rent_payer: solana_sdk::pubkey::Pubkey, 
        pub currency: solana_sdk::pubkey::Pubkey, 
        pub amount: u64, 
        pub price_offset: i32, 
        pub nfts_held: u32, 
        pub stats: PoolStats, 
        pub shared_escrow: solana_sdk::pubkey::Pubkey, 
        pub cosigner: solana_sdk::pubkey::Pubkey, 
        pub maker_broker: solana_sdk::pubkey::Pubkey, 
        pub max_taker_sell_count: u32, 
        pub config: PoolConfig, 
        pub reserved: [u8; 100], 
}