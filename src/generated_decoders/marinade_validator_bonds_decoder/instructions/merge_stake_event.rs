use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xe445a52e51cb9a1d6f062dd04f357739")]
pub struct MergeStakeEvent {
    pub config: solana_sdk::pubkey::Pubkey,
    pub staker_authority: solana_sdk::pubkey::Pubkey,
    pub destination_stake: solana_sdk::pubkey::Pubkey,
    pub destination_delegation: Option<DelegationInfo>,
    pub source_stake: solana_sdk::pubkey::Pubkey,
    pub source_delegation: Option<DelegationInfo>,
}
