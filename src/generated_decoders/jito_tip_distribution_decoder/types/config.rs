use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Config {
    pub authority: solana_sdk::pubkey::Pubkey,
    pub expired_funds_account: solana_sdk::pubkey::Pubkey,
    pub num_epochs_valid: u64,
    pub max_validator_commission_bps: u16,
    pub bump: u8,
}
