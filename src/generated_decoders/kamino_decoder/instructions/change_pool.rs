

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8ddd7beb230991c9")]
pub struct ChangePool{
}

pub struct ChangePoolInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub old_position: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub new_pool: solana_sdk::pubkey::Pubkey,
    pub strategy_reward_vault0_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub strategy_reward_vault1_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub strategy_reward_vault2_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for ChangePool {
    type ArrangedAccounts = ChangePoolInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let old_position = accounts.get(2)?;
        let base_vault_authority = accounts.get(3)?;
        let new_pool = accounts.get(4)?;
        let strategy_reward_vault0_or_base_vault_authority = accounts.get(5)?;
        let strategy_reward_vault1_or_base_vault_authority = accounts.get(6)?;
        let strategy_reward_vault2_or_base_vault_authority = accounts.get(7)?;

        Some(ChangePoolInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            old_position: old_position.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            new_pool: new_pool.pubkey,
            strategy_reward_vault0_or_base_vault_authority: strategy_reward_vault0_or_base_vault_authority.pubkey,
            strategy_reward_vault1_or_base_vault_authority: strategy_reward_vault1_or_base_vault_authority.pubkey,
            strategy_reward_vault2_or_base_vault_authority: strategy_reward_vault2_or_base_vault_authority.pubkey,
        })
    }
}