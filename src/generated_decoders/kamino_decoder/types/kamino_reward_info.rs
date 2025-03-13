use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct KaminoRewardInfo {
    pub decimals: u64,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_collateral_id: u64,
    pub last_issuance_ts: u64,
    pub reward_per_second: u64,
    pub amount_uncollected: u64,
    pub amount_issued_cumulative: u64,
    pub amount_available: u64,
}
