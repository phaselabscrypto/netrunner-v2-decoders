use super::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct SetPoolConfigParams {
    pub permissions: Permissions,
    pub oracle_authority: solana_sdk::pubkey::Pubkey,
    pub max_aum_usd: u64,
    pub staking_fee_share_bps: u64,
    pub vp_volume_factor: u8,
    pub staking_fee_boost_bps: [u64; 6],
    pub min_lp_price_usd: u64,
    pub max_lp_price_usd: u64,
    pub threshold_usd: u64,
}
