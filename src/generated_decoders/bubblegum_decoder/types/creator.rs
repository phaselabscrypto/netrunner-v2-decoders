use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::Creator as BubblegumCreator;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Creator {
    pub address: solana_sdk::pubkey::Pubkey,
    pub verified: bool,
    pub share: u8,
}

impl From<Creator> for BubblegumCreator {
    fn from(value: Creator) -> Self {
        Self {
            address: value.address,
            verified: value.verified,
            share: value.share,
        }
    }
}
