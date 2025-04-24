use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d1893a0ed84570fce")]
pub struct ProrataVaultParametersUpdatedEvent {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub max_buying_cap: u64,
    pub start_vesting_point: u64,
    pub end_vesting_point: u64,
}
