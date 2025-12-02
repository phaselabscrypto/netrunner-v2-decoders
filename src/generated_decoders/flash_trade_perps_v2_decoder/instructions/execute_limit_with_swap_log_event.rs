use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1de53269ad82738e4d")]
pub struct ExecuteLimitWithSwapLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub collateral_amount: u64,
    pub collateral_usd: u64,
    pub fee_amount: u64,
    pub reserve_custody_uid: u64,
    pub reserve_amount: u64,
    pub swap_fee_amount: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub limit_price: u64,
    pub limit_price_exponent: i32,
}
