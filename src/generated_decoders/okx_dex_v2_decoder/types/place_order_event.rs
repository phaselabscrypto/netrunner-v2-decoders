use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct PlaceOrderEvent {
    pub order_id: u64,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub input_token_mint: solana_sdk::pubkey::Pubkey,
    pub output_token_mint: solana_sdk::pubkey::Pubkey,
    pub making_amount: u64,
    pub expect_taking_amount: u64,
    pub min_return_amount: u64,
    pub create_ts: u64,
    pub deadline: u64,
    pub trade_fee: u64,
}
