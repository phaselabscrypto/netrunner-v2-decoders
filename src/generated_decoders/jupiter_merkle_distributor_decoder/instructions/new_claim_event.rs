use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df403e7973c653737")]
pub struct NewClaimEvent {
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub timestamp: i64,
}
