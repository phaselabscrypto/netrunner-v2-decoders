use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d136ab6ae0d74b047")]
pub struct RemoveCollateralAndSwapLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub delta_collateral_amount: u64,
    pub final_collateral_amount: u64,
    pub final_collateral_usd: u64,
    pub final_size_amount: u64,
    pub final_size_usd: u64,
    pub receive_custody_uid: u64,
    pub receive_amount: u64,
    pub swap_fee_amount: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
}
