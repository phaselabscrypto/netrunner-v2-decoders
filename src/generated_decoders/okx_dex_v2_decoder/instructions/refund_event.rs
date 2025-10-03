use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1db09fda3b5ed581da")]
pub struct RefundEvent {
    pub order_id: u64,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
