use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2533c582350f6312")]
pub struct FillOrderEvent {
    pub order_id: u64,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub output_token_mint: solana_sdk::pubkey::Pubkey,
    pub making_amount: u64,
    pub taking_amount: u64,
    pub update_ts: u64,
}
