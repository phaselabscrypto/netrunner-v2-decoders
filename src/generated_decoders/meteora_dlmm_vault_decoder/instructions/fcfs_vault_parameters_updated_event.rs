use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d4e70703ec1d1e7e2")]
pub struct FcfsVaultParametersUpdatedEvent {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub max_depositing_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
    pub depositing_point: u64,
    pub individual_depositing_cap: u64,
}
