

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct CollateralInfo {
    pub mint: solana_sdk::pubkey::Pubkey,
    pub lower_heuristic: u64,
    pub upper_heuristic: u64,
    pub exp_heuristic: u64,
    pub max_twap_divergence_bps: u64,
    pub scope_twap_price_chain: [u16; 4],
    pub scope_price_chain: [u16; 4],
    pub name: [u8; 32],
    pub max_age_price_seconds: u64,
    pub max_age_twap_seconds: u64,
    pub max_ignorable_amount_as_reward: u64,
    pub disabled: u8,
    pub padding0: [u8; 7],
    pub scope_staking_rate_chain: [u16; 4],
    pub padding: [u64; 8],
}
