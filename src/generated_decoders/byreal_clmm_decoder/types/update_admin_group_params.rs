use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateAdminGroupParams {
    pub fee_keeper: Option<solana_sdk::pubkey::Pubkey>,
    pub reward_config_manager: Option<solana_sdk::pubkey::Pubkey>,
    pub reward_claim_manager: Option<solana_sdk::pubkey::Pubkey>,
    pub pool_manager: Option<solana_sdk::pubkey::Pubkey>,
    pub emergency_manager: Option<solana_sdk::pubkey::Pubkey>,
    pub normal_manager: Option<solana_sdk::pubkey::Pubkey>,
}
