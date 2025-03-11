
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xae6e2777526aa966")] 
pub struct Strategy { 
        pub reserve: solana_sdk::pubkey::Pubkey, 
        pub collateral_vault: solana_sdk::pubkey::Pubkey, 
        pub strategy_type: StrategyType, 
        pub current_liquidity: u64, 
        pub bumps: [u8; 10], 
        pub vault: solana_sdk::pubkey::Pubkey, 
        pub is_disable: u8, 
}