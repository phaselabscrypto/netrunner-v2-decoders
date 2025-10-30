use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LockerParams {
    pub max_stake_vote_multiplier: u8,
    pub min_stake_duration: u64,
    pub max_stake_duration: u64,
    pub proposal_activation_min_votes: u64,
}
