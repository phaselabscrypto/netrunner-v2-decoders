use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct NewDistributorParams {
    pub version: u64,
    pub root: [u8; 32],
    pub total_claim: u64,
    pub max_num_nodes: u64,
    pub start_vesting_ts: i64,
    pub end_vesting_ts: i64,
    pub clawback_start_ts: i64,
    pub activation_point: u64,
    pub activation_type: u8,
    pub closable: bool,
    pub total_bonus: u64,
    pub bonus_vesting_duration: u64,
    pub claim_type: u8,
    pub operator: solana_sdk::pubkey::Pubkey,
    pub locker: solana_sdk::pubkey::Pubkey,
}
