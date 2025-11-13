use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d244714468aedadb3")]
pub struct PlaceLimitOrderLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub limit_price: u64,
    pub limit_price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub reserve_price: u64,
    pub reserve_price_exponent: i32,
    pub reserve_amount: u64,
    pub reserve_usd: u64,
    pub stop_loss_price: u64,
    pub stop_loss_price_exponent: i32,
    pub take_profit_price: u64,
    pub take_profit_price_exponent: i32,
    pub receive_custody_uid: u8,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
}
