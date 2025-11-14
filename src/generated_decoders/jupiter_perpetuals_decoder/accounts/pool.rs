use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xf19a6d0411b16dbc")]
pub struct Pool {
    pub name: String,
    pub custodies: Vec<solana_sdk::pubkey::Pubkey>,
    pub aum_usd: u128,
    pub limit: Limit,
    pub fees: Fees,
    pub pool_apr: PoolApr,
    pub max_request_execution_sec: i64,
    pub bump: u8,
    pub lp_token_bump: u8,
    pub inception_time: i64,
    pub parameter_update_oracle: Secp256k1Pubkey,
    pub aum_usd_updated_at: i64,
    pub max_trigger_price_diff_bps: u64,
}
