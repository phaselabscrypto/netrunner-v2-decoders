use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d5d4f16851d8c5c9d")]
pub struct IncreaseSizeLogV4Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub entry_price: u64,
    pub entry_price_exponent: i32,
    pub delta_size_amount: u64,
    pub delta_size_usd: u64,
    pub final_size_amount: u64,
    pub final_size_usd: u64,
    pub final_collateral_amount: u64,
    pub final_collateral_usd: u64,
    pub fee_amount: u64,
    pub fee_rebate_amount: u64,
    pub entry_fee_amount: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub is_degen: bool,
    pub padding: [u8; 31],
}
