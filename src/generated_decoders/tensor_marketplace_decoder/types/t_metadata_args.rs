
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct TMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TTokenStandard>,
    pub collection: Option<TCollection>,
    pub uses: Option<TUses>,
    pub token_program_version: TTokenProgramVersion,
    pub creator_shares: Vec<u8>,
    pub creator_verified: Vec<bool>,
}
