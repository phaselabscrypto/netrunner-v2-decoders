use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df075ccfe5996845e")]
pub struct SetAdminEvent {
    pub admin: solana_sdk::pubkey::Pubkey,
}
