use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitCompoundingParams {
    pub fee_share_bps: u64,
    pub metadata_title: String,
    pub metadata_symbol: String,
    pub metadata_uri: String,
}
