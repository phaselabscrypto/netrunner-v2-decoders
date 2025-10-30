use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d90acd15690575473")]
pub struct ClaimedEvent {
    pub claimant: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
