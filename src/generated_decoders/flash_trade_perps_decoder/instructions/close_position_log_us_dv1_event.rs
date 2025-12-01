use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d2c644321ff017564")]
pub struct ClosePositionLogUsDv1Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trade_id: i64,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub duration: i64,
    pub exit_price: u64,
    pub exit_price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub collateral_usd: u64,
    pub profit_usd: u64,
    pub loss_usd: u64,
    pub fee_usd: u64,
    pub rebate_usd: u64,
    pub discount_usd: u64,
    pub exit_fee_usd: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub padding: [u64; 4],
}
