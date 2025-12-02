use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3cc77fbe025e453e")]
pub struct SwapAndAddCollateralLogUsDv1Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trade_id: i64,
    pub input_custody_uid: u8,
    pub input_price: u64,
    pub input_price_exponent: i32,
    pub delta_input_amount: u64,
    pub delta_collateral_usd: u64,
    pub final_collateral_usd: u64,
    pub final_size_amount: u64,
    pub final_size_usd: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub padding: [u64; 4],
}
