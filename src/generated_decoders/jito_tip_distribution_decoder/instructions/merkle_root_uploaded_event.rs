

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5ee9ec3134e0b5a7")]
pub struct MerkleRootUploadedEvent{
    pub merkle_root_upload_authority: solana_sdk::pubkey::Pubkey,
    pub tip_distribution_account: solana_sdk::pubkey::Pubkey,
}
