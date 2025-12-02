use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0xdbbed53700e3c69a")]
pub struct Market {
    pub pool: solana_sdk::pubkey::Pubkey,
    pub target_custody: solana_sdk::pubkey::Pubkey,
    pub collateral_custody: solana_sdk::pubkey::Pubkey,
    pub side: Side,
    pub correlation: bool,
    pub max_payoff_bps: u64,
    pub permissions: MarketPermissions,
    pub degen_exposure_usd: u64,
    pub collective_position: PositionStats,
    pub target_custody_uid: u8,
    pub padding: [u8; 7],
    pub collateral_custody_uid: u8,
    pub padding2: [u8; 7],
    pub bump: u8,
}
