use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::Collection as BubblegumCollection;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Collection {
    pub verified: bool,
    pub key: solana_sdk::pubkey::Pubkey,
}

impl From<Collection> for BubblegumCollection {
    fn from(value: Collection) -> Self {
        Self {
            verified: true,
            key: value.key,
        }
    }
}
