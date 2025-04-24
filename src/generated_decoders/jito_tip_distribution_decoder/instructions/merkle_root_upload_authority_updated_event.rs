use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d539d3aa5c8ab086a")]
pub struct MerkleRootUploadAuthorityUpdatedEvent {
    pub old_authority: solana_sdk::pubkey::Pubkey,
    pub new_authority: solana_sdk::pubkey::Pubkey,
}
