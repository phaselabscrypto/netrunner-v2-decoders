use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RenameFlpParams {
    pub flag: u64,
    pub lp_token_name: String,
    pub lp_token_symbol: String,
    pub lp_token_uri: String,
}
