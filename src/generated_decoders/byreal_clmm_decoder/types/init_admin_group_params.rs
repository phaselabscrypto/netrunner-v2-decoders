use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct InitAdminGroupParams {
    pub fee_keeper: solana_sdk::pubkey::Pubkey,
    pub reward_config_manager: solana_sdk::pubkey::Pubkey,
    pub reward_claim_manager: solana_sdk::pubkey::Pubkey,
    pub pool_manager: solana_sdk::pubkey::Pubkey,
    pub emergency_manager: solana_sdk::pubkey::Pubkey,
    pub normal_manager: solana_sdk::pubkey::Pubkey,
}
