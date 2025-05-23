use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Creator {
    pub address: solana_sdk::pubkey::Pubkey,
    pub verified: bool,
    pub percentage_share: u8,
}
