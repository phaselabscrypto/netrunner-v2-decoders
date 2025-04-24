use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::TokenStandard as BubblegumTokenStandard;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TokenStandard {
    NonFungible,
    FungibleAsset,
    Fungible,
    NonFungibleEdition,
}

impl From<TokenStandard> for BubblegumTokenStandard {
    fn from(value: TokenStandard) -> Self {
        match value {
            TokenStandard::NonFungible => BubblegumTokenStandard::NonFungible,
            TokenStandard::FungibleAsset => BubblegumTokenStandard::FungibleAsset,
            TokenStandard::Fungible => BubblegumTokenStandard::Fungible,
            TokenStandard::NonFungibleEdition => BubblegumTokenStandard::NonFungibleEdition,
        }
    }
}
