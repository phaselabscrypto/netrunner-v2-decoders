use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xaeae8ec12f4deb41")]
pub struct AddKaminoRewards {
    pub kamino_reward_index: u64,
    pub amount: u64,
}

pub struct AddKaminoRewardsInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub reward_ata: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddKaminoRewards {
    type ArrangedAccounts = AddKaminoRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let reward_mint = accounts.get(2)?;
        let reward_vault = accounts.get(3)?;
        let base_vault_authority = accounts.get(4)?;
        let reward_ata = accounts.get(5)?;
        let token_program = accounts.get(6)?;

        Some(AddKaminoRewardsInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            reward_ata: reward_ata.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
