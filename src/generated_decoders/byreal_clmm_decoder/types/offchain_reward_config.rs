use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct OffchainRewardConfig {
    pub pool_id: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub vault_bump: [u8; 1],
    pub reward_mint_vec: Vec<solana_sdk::pubkey::Pubkey>,
}
