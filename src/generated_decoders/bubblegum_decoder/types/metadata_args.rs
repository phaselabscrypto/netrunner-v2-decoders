use super::*;

use carbon_core::{borsh, CarbonDeserialize};
use mpl_bubblegum::types::MetadataArgs as BubblegumMetadataArgs;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub token_program_version: TokenProgramVersion,
    pub creators: Vec<Creator>,
}

trait OptionConvert<T> {
    fn convert(self) -> Option<T>;
}

impl<T, U> OptionConvert<U> for Option<T>
where
    T: Into<U>,
{
    fn convert(self) -> Option<U> {
        self.map(|t| t.into())
    }
}

impl From<MetadataArgs> for BubblegumMetadataArgs {
    fn from(data: MetadataArgs) -> Self {
        Self {
            name: data.name,
            symbol: data.symbol,
            uri: data.uri,
            seller_fee_basis_points: data.seller_fee_basis_points,
            primary_sale_happened: data.primary_sale_happened,
            is_mutable: data.is_mutable,
            edition_nonce: data.edition_nonce,
            token_standard: data.token_standard.convert(),
            collection: data.collection.convert(),
            uses: data.uses.convert(),
            token_program_version: data.token_program_version.into(),
            creators: data.creators.into_iter().map(|c| c.into()).collect(),
        }
    }
}
