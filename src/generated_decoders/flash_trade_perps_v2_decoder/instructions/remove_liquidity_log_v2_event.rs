use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d3f86321000263773")]
pub struct RemoveLiquidityLogV2Event {
    pub pool_name: String,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub custody_uid: u64,
    pub lp_amount_in: u64,
    pub amount_out: u64,
    pub fee_amount: u64,
    pub lp_price_usd: u64,
    pub token_out_price: u64,
    pub token_out_price_exponent: i32,
}
