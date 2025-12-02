use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de48310c984f9f897")]
pub struct OpenPositionLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub price_usd: u64,
    pub size_amount: u64,
    pub size_usd: u64,
    pub collateral_amount: u64,
    pub collateral_usd: u64,
    pub fee_amount: u64,
}
