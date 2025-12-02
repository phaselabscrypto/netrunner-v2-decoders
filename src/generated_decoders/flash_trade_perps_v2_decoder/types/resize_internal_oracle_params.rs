use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ResizeInternalOracleParams {
    pub ext_oracle: solana_sdk::pubkey::Pubkey,
}
