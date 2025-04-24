use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xd308e82b02987577")]
pub struct Vault {
    pub enabled: u8,
    pub bumps: VaultBumps,
    pub total_amount: u64,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub token_mint: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub strategies: [solana_sdk::pubkey::Pubkey; 30],
    pub base: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub locked_profit_tracker: LockedProfitTracker,
}
