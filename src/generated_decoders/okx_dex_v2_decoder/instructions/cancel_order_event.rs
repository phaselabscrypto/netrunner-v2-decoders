use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1dae428d1104e0a24d")]
pub struct CancelOrderEvent {
    pub order_id: u64,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub update_ts: u64,
}
