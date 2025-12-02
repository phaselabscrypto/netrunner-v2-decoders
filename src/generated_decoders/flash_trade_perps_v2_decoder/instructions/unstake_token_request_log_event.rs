use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1da691a33def3ab5f2")]
pub struct UnstakeTokenRequestLogEvent {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub token_stake: solana_sdk::pubkey::Pubkey,
    pub unstake_amount: u64,
    pub current_timestamp: i64,
    pub last_updated_timestamp: i64,
    pub level: u8,
    pub active_stake_amount: u64,
}
