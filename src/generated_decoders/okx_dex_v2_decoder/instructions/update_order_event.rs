use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d37182ff069f51e87")]
pub struct UpdateOrderEvent {
    pub order_id: u64,
    pub maker: solana_sdk::pubkey::Pubkey,
    pub expect_taking_amount: u64,
    pub min_return_amount: u64,
    pub deadline: u64,
    pub update_ts: u64,
    pub increase_fee: u64,
}
