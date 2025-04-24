use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Allowlist {
    pub kind: u8,
    pub value: solana_sdk::pubkey::Pubkey,
}
