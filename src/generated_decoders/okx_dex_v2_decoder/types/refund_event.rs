use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RefundEvent {
    pub order_id: u64,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
}
