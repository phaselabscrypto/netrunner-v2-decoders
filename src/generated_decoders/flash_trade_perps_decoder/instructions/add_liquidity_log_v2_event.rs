use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df356c9ffa1fe2067")]
pub struct AddLiquidityLogV2Event {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub custody_uid: u64,
    pub amount_in: u64,
    pub lp_amount_out: u64,
    pub fee_amount: u64,
    pub lp_price_usd: u64,
    pub token_in_price: u64,
    pub token_in_price_exponent: i32,
}
