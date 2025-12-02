use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct CompoundingStats {
    pub active_amount: u64,
    pub total_supply: u64,
    pub reward_snapshot: u128,
    pub fee_share_bps: u64,
    pub last_compound_time: i64,
}
