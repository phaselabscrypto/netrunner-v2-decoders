use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1df5245e83fbd47ffa")]
pub struct DistributeTokenRewardLogEvent {
    pub amount: u64,
    pub epoch_count: u32,
    pub padding: [u64; 2],
}
