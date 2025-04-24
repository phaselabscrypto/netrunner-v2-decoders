use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct DelegationInfo {
    pub voter_pubkey: solana_sdk::pubkey::Pubkey,
    pub stake: u64,
    pub activation_epoch: u64,
    pub deactivation_epoch: u64,
}
