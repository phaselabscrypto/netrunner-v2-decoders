use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9194179ed3b7abe3")]
pub struct ExecuteTriggerOrderLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub duration: i64,
    pub exit_price: u64,
    pub exit_price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub collateral_amount: u64,
    pub profit_usd: u64,
    pub loss_usd: u64,
    pub fee_amount: u64,
    pub fee_rebate_amount: u64,
    pub exit_fee_amount: u64,
    pub is_stop_loss: bool,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub trigger_price: u64,
    pub trigger_price_exponent: i32,
}
