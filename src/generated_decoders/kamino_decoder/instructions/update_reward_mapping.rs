

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xcb2525601755e92a")]
pub struct UpdateRewardMapping{
    pub reward_index: u8,
    pub collateral_token: u8,
}

pub struct UpdateRewardMappingInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateRewardMapping {
    type ArrangedAccounts = UpdateRewardMappingInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let pool = accounts.get(3)?;
        let reward_mint = accounts.get(4)?;
        let reward_vault = accounts.get(5)?;
        let base_vault_authority = accounts.get(6)?;
        let token_infos = accounts.get(7)?;
        let system_program = accounts.get(8)?;
        let rent = accounts.get(9)?;
        let token_program = accounts.get(10)?;

        Some(UpdateRewardMappingInstructionAccounts {
            payer: payer.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            token_infos: token_infos.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}