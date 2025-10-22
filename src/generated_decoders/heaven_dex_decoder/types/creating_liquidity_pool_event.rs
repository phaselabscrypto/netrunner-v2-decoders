use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CreatingLiquidityPoolEvent {
    pub id: solana_sdk::pubkey::Pubkey,
    pub base: solana_sdk::pubkey::Pubkey,
    pub quote: solana_sdk::pubkey::Pubkey,
    pub base_amount: u64,
    pub quote_amount: u64,
}
