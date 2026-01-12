use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d9dfdaad5387200e7")]
pub struct ExecuteLimitOrderLogUsDv1Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trade_id: i64,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub delta_size_amount: u64,
    pub delta_size_usd: u64,
    pub final_size_amount: u64,
    pub final_size_usd: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub delta_collateral_amount: u64,
    pub delta_collateral_usd: u64,
    pub final_collateral_usd: u64,
    pub fee_usd: u64,
    pub rebate_usd: u64,
    pub discount_usd: u64,
    pub entry_fee_usd: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub limit_price: u64,
    pub limit_price_exponent: i32,
    pub padding: [u64; 4],
}
