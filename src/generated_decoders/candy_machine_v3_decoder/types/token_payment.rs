use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct TokenPayment {
    pub amount: u64,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination_ata: solana_sdk::pubkey::Pubkey,
}
