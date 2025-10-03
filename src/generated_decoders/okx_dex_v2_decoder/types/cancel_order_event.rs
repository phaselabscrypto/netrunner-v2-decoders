use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CancelOrderEvent {
    pub order_id: u64,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub update_ts: u64,
}
