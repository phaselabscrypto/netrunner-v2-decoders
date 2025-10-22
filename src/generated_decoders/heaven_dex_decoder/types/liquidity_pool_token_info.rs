use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LiquidityPoolTokenInfo {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub decimals: u8,
    pub owner: solana_sdk::pubkey::Pubkey,
}
