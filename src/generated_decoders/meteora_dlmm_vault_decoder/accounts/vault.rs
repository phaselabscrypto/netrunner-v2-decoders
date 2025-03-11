
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0xd308e82b02987577")] 
pub struct Vault { 
        pub pool: solana_sdk::pubkey::Pubkey, 
        pub token_vault: solana_sdk::pubkey::Pubkey, 
        pub token_out_vault: solana_sdk::pubkey::Pubkey, 
        pub quote_mint: solana_sdk::pubkey::Pubkey, 
        pub base_mint: solana_sdk::pubkey::Pubkey, 
        pub base: solana_sdk::pubkey::Pubkey, 
        pub owner: solana_sdk::pubkey::Pubkey, 
        pub max_buying_cap: u64, 
        pub total_deposit: u64, 
        pub total_escrow: u64, 
        pub swapped_amount: u64, 
        pub bought_token: u64, 
        pub total_refund: u64, 
        pub total_claimed_token: u64, 
        pub start_vesting_point: u64, 
        pub end_vesting_point: u64, 
        pub bump: u8, 
        pub pool_type: u8, 
        pub vault_mode: u8, 
        pub padding0: [u8; 5], 
        pub max_depositing_cap: u64, 
        pub individual_depositing_cap: u64, 
        pub depositing_point: u64, 
        pub escrow_fee: u64, 
        pub total_escrow_fee: u64, 
        pub whitelist_mode: u8, 
        pub activation_type: u8, 
        pub padding1: [u8; 6], 
        pub vault_authority: solana_sdk::pubkey::Pubkey, 
        pub padding: [u128; 5], 
}