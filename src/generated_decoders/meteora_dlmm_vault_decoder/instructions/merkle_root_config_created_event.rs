

use carbon_core::{borsh, CarbonDeserialize};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xe445a52e51cb9a1d79702a4c90838e5a")]
pub struct MerkleRootConfigCreatedEvent{
    pub admin: solana_sdk::pubkey::Pubkey,
    pub config: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub version: u64,
    pub root: [u8; 32],
}
