use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SwapEvent {
    pub amm: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_amount: u64,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub output_amount: u64,
}
