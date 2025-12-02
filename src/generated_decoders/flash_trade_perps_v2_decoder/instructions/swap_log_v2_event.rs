use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d66cad326874127df")]
pub struct SwapLogV2Event {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub custody_in_uid: u64,
    pub custody_out_uid: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_in_amount: u64,
    pub fee_out_amount: u64,
    pub in_oracle_account_time: i64,
    pub in_oracle_account_type: u8,
    pub in_oracle_account_price: u64,
    pub in_oracle_account_price_exponent: i32,
    pub out_oracle_account_time: i64,
    pub out_oracle_account_type: u8,
    pub out_oracle_account_price: u64,
    pub out_oracle_account_price_exponent: i32,
}
