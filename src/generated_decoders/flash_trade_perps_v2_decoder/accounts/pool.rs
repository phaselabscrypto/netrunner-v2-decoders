use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xf19a6d0411b16dbc")]
pub struct Pool {
    pub name: String,
    pub permissions: Permissions,
    pub inception_time: i64,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub oracle_authority: solana_sdk::pubkey::Pubkey,
    pub staked_lp_vault: solana_sdk::pubkey::Pubkey,
    pub reward_custody: solana_sdk::pubkey::Pubkey,
    pub custodies: Vec<solana_sdk::pubkey::Pubkey>,
    pub ratios: Vec<TokenRatios>,
    pub markets: Vec<solana_sdk::pubkey::Pubkey>,
    pub max_aum_usd: u64,
    pub buffer: u64,
    pub raw_aum_usd: u64,
    pub equity_usd: u64,
    pub total_staked: StakeStats,
    pub staking_fee_share_bps: u64,
    pub bump: u8,
    pub lp_mint_bump: u8,
    pub staked_lp_vault_bump: u8,
    pub vp_volume_factor: u8,
    pub unique_custody_count: u8,
    pub padding: [u8; 3],
    pub staking_fee_boost_bps: [u64; 6],
    pub compounding_mint: solana_sdk::pubkey::Pubkey,
    pub compounding_lp_vault: solana_sdk::pubkey::Pubkey,
    pub compounding_stats: CompoundingStats,
    pub compounding_mint_bump: u8,
    pub compounding_lp_vault_bump: u8,
    pub min_lp_price_usd: u64,
    pub max_lp_price_usd: u64,
    pub lp_price: u64,
    pub compounding_lp_price: u64,
    pub last_updated_timestamp: i64,
    pub fees_obligation_usd: u64,
    pub rebate_obligation_usd: u64,
    pub threshold_usd: u64,
}
