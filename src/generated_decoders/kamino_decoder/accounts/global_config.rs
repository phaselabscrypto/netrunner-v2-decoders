
 
use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x95089ccaa0fcb0d9")] 
pub struct GlobalConfig { 
        pub emergency_mode: u64, 
        pub block_deposit: u64, 
        pub block_invest: u64, 
        pub block_withdraw: u64, 
        pub block_collect_fees: u64, 
        pub block_collect_rewards: u64, 
        pub block_swap_rewards: u64, 
        pub block_swap_uneven_vaults: u32, 
        pub block_emergency_swap: u32, 
        pub min_withdrawal_fee_bps: u64, 
        pub scope_program_id: solana_sdk::pubkey::Pubkey, 
        pub scope_price_id: solana_sdk::pubkey::Pubkey, 
        pub swap_rewards_discount_bps: [u64; 256], 
        pub actions_authority: solana_sdk::pubkey::Pubkey, 
        pub admin_authority: solana_sdk::pubkey::Pubkey, 
        pub treasury_fee_vaults: [solana_sdk::pubkey::Pubkey; 256], 
        pub token_infos: solana_sdk::pubkey::Pubkey, 
        pub block_local_admin: u64, 
        pub min_performance_fee_bps: u64, 
        pub min_swap_uneven_slippage_tolerance_bps: u64, 
        pub min_reference_price_slippage_tolerance_bps: u64, 
        pub actions_after_rebalance_delay_seconds: u64, 
        pub treasury_fee_vault_receiver: solana_sdk::pubkey::Pubkey, 
        pub padding: [u64; 2035], 
}