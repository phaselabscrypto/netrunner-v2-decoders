use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitializeVaultWithConfigParams {
    pub pool_type: u8,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub whitelist_mode: u8,
}
