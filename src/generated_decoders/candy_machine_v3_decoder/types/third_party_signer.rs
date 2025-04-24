use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ThirdPartySigner {
    pub signer_key: solana_sdk::pubkey::Pubkey,
}
