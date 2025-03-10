
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf19a6d0411b16dbc")] 
pub struct Pool { 
        pub lp_mint: solana_sdk::pubkey::Pubkey, 
        pub token_a_mint: solana_sdk::pubkey::Pubkey, 
        pub token_b_mint: solana_sdk::pubkey::Pubkey, 
        pub a_vault: solana_sdk::pubkey::Pubkey, 
        pub b_vault: solana_sdk::pubkey::Pubkey, 
        pub a_vault_lp: solana_sdk::pubkey::Pubkey, 
        pub b_vault_lp: solana_sdk::pubkey::Pubkey, 
        pub a_vault_lp_bump: u8, 
        pub enabled: bool, 
        pub protocol_token_a_fee: solana_sdk::pubkey::Pubkey, 
        pub protocol_token_b_fee: solana_sdk::pubkey::Pubkey, 
        pub fee_last_updated_at: u64, 
        pub padding0: [u8; 24], 
        pub fees: PoolFees, 
        pub pool_type: PoolType, 
        pub stake: solana_sdk::pubkey::Pubkey, 
        pub total_locked_lp: u64, 
        pub bootstrapping: Bootstrapping, 
        pub partner_info: PartnerInfo, 
        pub padding: Padding, 
        pub curve_type: CurveType, 
}