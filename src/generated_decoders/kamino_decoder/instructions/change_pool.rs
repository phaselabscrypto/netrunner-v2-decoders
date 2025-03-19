use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8ddd7beb230991c9")]
pub struct ChangePool {}

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

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, strategy, old_position, base_vault_authority, new_pool, strategy_reward_vault0_or_base_vault_authority, strategy_reward_vault1_or_base_vault_authority, strategy_reward_vault2_or_base_vault_authority, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(ChangePoolInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            old_position: old_position.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            new_pool: new_pool.pubkey,
            strategy_reward_vault0_or_base_vault_authority:
                strategy_reward_vault0_or_base_vault_authority.pubkey,
            strategy_reward_vault1_or_base_vault_authority:
                strategy_reward_vault1_or_base_vault_authority.pubkey,
            strategy_reward_vault2_or_base_vault_authority:
                strategy_reward_vault2_or_base_vault_authority.pubkey,
        })
    }
}
