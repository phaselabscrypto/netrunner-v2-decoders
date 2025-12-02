use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1daf6c91121f338843")]
pub struct SetTokenRewardLogEvent {
    pub token_stake: solana_sdk::pubkey::Pubkey,
    pub amount: u64,
    pub epoch_count: u32,
    pub padding: [u64; 2],
}
