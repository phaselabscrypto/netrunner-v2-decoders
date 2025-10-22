use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x8080ea1e3dacbc7b")]
pub struct AmmAdminGroup {
    pub fee_keeper: solana_sdk::pubkey::Pubkey,
    pub reward_config_manager: solana_sdk::pubkey::Pubkey,
    pub reward_claim_manager: solana_sdk::pubkey::Pubkey,
    pub pool_manager: solana_sdk::pubkey::Pubkey,
    pub emergency_manager: solana_sdk::pubkey::Pubkey,
    pub normal_manager: solana_sdk::pubkey::Pubkey,
    pub pad: [solana_sdk::pubkey::Pubkey; 6],
}
