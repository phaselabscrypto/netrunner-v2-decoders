use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x01b830515d833f91")]
pub struct Custody {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub token_account: solana_sdk::pubkey::Pubkey,
    pub decimals: u8,
    pub is_stable: bool,
    pub depeg_adjustment: bool,
    pub is_virtual: bool,
    pub distribute_rewards: bool,
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub fees: Fees,
    pub borrow_rate: BorrowRateParams,
    pub reward_threshold: u64,
    pub assets: Assets,
    pub fees_stats: FeesStats,
    pub borrow_rate_state: BorrowRateState,
    pub bump: u8,
    pub token_account_bump: u8,
    pub token22: bool,
    pub uid: u8,
    pub reserved_amount: u64,
    pub min_reserve_usd: u64,
    pub limit_price_buffer_bps: u64,
    pub padding: [u8; 32],
}
