

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct InitializeProrataVaultParams {
    pub pool_type: u8,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub depositing_point: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub max_buying_cap: u64,
    pub escrow_fee: u64,
    pub whitelist_mode: u8,
}
