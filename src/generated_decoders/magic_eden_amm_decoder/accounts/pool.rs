
use super::super::types::*;
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xf19a6d0411b16dbc")] 
pub struct Pool { 
        pub spot_price: u64, 
        pub curve_type: u8, 
        pub curve_delta: u64, 
        pub reinvest_fulfill_buy: bool, 
        pub reinvest_fulfill_sell: bool, 
        pub expiry: i64, 
        pub lp_fee_bp: u16, 
        pub referral: solana_sdk::pubkey::Pubkey, 
        pub referral_bp: u16, 
        pub buyside_creator_royalty_bp: u16, 
        pub cosigner_annotation: [u8; 32], 
        pub sellside_asset_amount: u64, 
        pub lp_fee_earned: u64, 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub cosigner: solana_sdk::pubkey::Pubkey, 
        pub uuid: solana_sdk::pubkey::Pubkey, 
        pub payment_mint: solana_sdk::pubkey::Pubkey, 
        pub allowlists: [Allowlist; 6], 
        pub buyside_payment_amount: u64, 
        pub shared_escrow_account: solana_sdk::pubkey::Pubkey, 
        pub shared_escrow_count: u64, 
}