use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x1ca762bf68526cc4")]
pub struct Perpetuals {
    pub permissions: Permissions,
    pub pools: Vec<solana_sdk::pubkey::Pubkey>,
    pub collections: Vec<solana_sdk::pubkey::Pubkey>,
    pub voltage_multiplier: VoltageMultiplier,
    pub trading_discount: [u64; 6],
    pub referral_rebate: [u64; 6],
    pub default_rebate: u64,
    pub inception_time: i64,
    pub transfer_authority_bump: u8,
    pub perpetuals_bump: u8,
    pub trade_limit: u8,
    pub trigger_order_limit: u8,
    pub rebate_limit_usd: u32,
}
