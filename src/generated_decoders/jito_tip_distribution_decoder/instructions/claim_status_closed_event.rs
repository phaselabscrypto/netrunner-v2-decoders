use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dbc8fede5c0b6a476")]
pub struct ClaimStatusClosedEvent {
    pub claim_status_payer: solana_sdk::pubkey::Pubkey,
    pub claim_status_account: solana_sdk::pubkey::Pubkey,
}
