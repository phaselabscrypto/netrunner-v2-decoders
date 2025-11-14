use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d89647ec5c2593fc0")]
pub struct SwapAndOpenLogEvent {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub receiving_custody_uid: u64,
    pub collateral_custody_uid: u64,
    pub target_custody_uid: u64,
    pub amount_in: u64,
    pub swap_amount_out: u64,
    pub swap_fee_amount: u64,
    pub market: solana_sdk::pubkey::Pubkey,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub size_amount: u64,
    pub size_usd: u64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub collateral_amount: u64,
    pub collateral_usd: u64,
    pub position_fee_amount: u64,
    pub fee_rebate_amount: u64,
    pub receiving_oracle_account_time: i64,
    pub receiving_oracle_account_type: u8,
    pub receiving_oracle_account_price: u64,
    pub receiving_oracle_account_price_exponent: i32,
    pub collateral_oracle_account_time: i64,
    pub collateral_oracle_account_type: u8,
    pub collateral_oracle_account_price: u64,
    pub collateral_oracle_account_price_exponent: i32,
    pub target_oracle_account_time: i64,
    pub target_oracle_account_type: u8,
    pub target_oracle_account_price: u64,
    pub target_oracle_account_price_exponent: i32,
}
