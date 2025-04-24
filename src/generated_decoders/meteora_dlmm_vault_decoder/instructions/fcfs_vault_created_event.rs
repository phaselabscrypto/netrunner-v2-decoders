use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4999a56797b6b888")]
pub struct FcfsVaultCreatedEvent {
    pub base_mint: solana_sdk::pubkey::Pubkey,
    pub quote_mint: solana_sdk::pubkey::Pubkey,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub max_depositing_cap: u64,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub pool_type: u8,
    pub depositing_point: u64,
    pub individual_depositing_cap: u64,
    pub escrow_fee: u64,
    pub activation_type: u8,
}
