use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d78da305bce058070")]
pub struct RefreshStakeLogEvent {
    pub pool_name: String,
    pub reward_per_lp_staked: u64,
}
