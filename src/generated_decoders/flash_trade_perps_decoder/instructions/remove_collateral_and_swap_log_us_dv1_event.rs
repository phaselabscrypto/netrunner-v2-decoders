use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d39c69ad25b5a3a30")]
pub struct RemoveCollateralAndSwapLogUsDv1Event {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub market: solana_sdk::pubkey::Pubkey,
    pub trade_id: i64,
    pub collateral_price: u64,
    pub collateral_price_exponent: i32,
    pub delta_collateral_usd: u64,
    pub final_collateral_usd: u64,
    pub final_size_amount: u64,
    pub final_size_usd: u64,
    pub receive_custody_uid: u8,
    pub receive_amount: u64,
    pub oracle_account_time: i64,
    pub oracle_account_type: u8,
    pub oracle_account_price: u64,
    pub oracle_account_price_exponent: i32,
    pub padding: [u64; 4],
}
