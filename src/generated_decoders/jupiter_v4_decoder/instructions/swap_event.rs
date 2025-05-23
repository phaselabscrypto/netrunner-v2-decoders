use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d516ce3becdd00ac4")]
pub struct SwapEvent {
    pub amm: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub input_amount: u64,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub output_amount: u64,
}
