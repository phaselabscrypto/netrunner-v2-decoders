

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct DelegationInfo {
    pub voter_pubkey: solana_sdk::pubkey::Pubkey,
    pub stake: u64,
    pub activation_epoch: u64,
    pub deactivation_epoch: u64,
}
