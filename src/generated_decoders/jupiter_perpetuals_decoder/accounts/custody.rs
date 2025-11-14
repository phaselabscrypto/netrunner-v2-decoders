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
    pub oracle: OracleParams,
    pub pricing: PricingParams,
    pub permissions: Permissions,
    pub target_ratio_bps: u64,
    pub assets: Assets,
    pub funding_rate_state: FundingRateState,
    pub bump: u8,
    pub token_account_bump: u8,
    pub increase_position_bps: u64,
    pub decrease_position_bps: u64,
    pub max_position_size_usd: u64,
    pub doves_oracle: solana_sdk::pubkey::Pubkey,
    pub jump_rate_state: JumpRateState,
    pub doves_ag_oracle: solana_sdk::pubkey::Pubkey,
    pub price_impact_buffer: PriceImpactBuffer,
    pub borrow_lend_parameters: BorrowLendParams,
    pub borrows_funding_rate_state: FundingRateState,
    pub debt: u128,
    pub borrow_lend_interests_accured: u128,
    pub borrow_limit_in_token_amount: u64,
    pub min_interest_fee_bps: u64,
    pub min_interest_fee_grace_period_seconds: u64,
    pub total_staked_amount_lamports: u64,
    pub max_total_staked_amount_lamports: u64,
    pub external_swap_fee_multiplier_bps: u64,
}
