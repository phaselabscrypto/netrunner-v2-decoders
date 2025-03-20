
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf7ede3f5d7c3de46")] 
pub struct PoolState { 
        pub bump: u8, 
        pub amm_config: solana_sdk::pubkey::Pubkey, 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub token_mint0: solana_sdk::pubkey::Pubkey, 
        pub token_mint1: solana_sdk::pubkey::Pubkey, 
        pub token_vault0: solana_sdk::pubkey::Pubkey, 
        pub token_vault1: solana_sdk::pubkey::Pubkey, 
        pub observation_key: solana_sdk::pubkey::Pubkey, 
        pub mint_decimals0: u8, 
        pub mint_decimals1: u8, 
        pub tick_spacing: u16, 
        pub liquidity: u128, 
        pub sqrt_price_x64: u128, 
        pub tick_current: i32, 
        pub observation_index: u16, 
        pub observation_update_duration: u16, 
        pub fee_growth_global0_x64: u128, 
        pub fee_growth_global1_x64: u128, 
        pub protocol_fees_token0: u64, 
        pub protocol_fees_token1: u64, 
        pub swap_in_amount_token0: u128, 
        pub swap_out_amount_token1: u128, 
        pub swap_in_amount_token1: u128, 
        pub swap_out_amount_token0: u128, 
        pub status: u8, 
        pub padding: [u8; 7], 
        pub reward_infos: [RewardInfo; 3], 
        pub tick_array_bitmap: [u64; 16], 
        pub total_fees_token0: u64, 
        pub total_fees_claimed_token0: u64, 
        pub total_fees_token1: u64, 
        pub total_fees_claimed_token1: u64, 
        pub fund_fees_token0: u64, 
        pub fund_fees_token1: u64, 
        pub open_time: u64, 
        pub padding1: [u64; 25], 
        pub padding2: [u64; 32], 
}