use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcb2525601755e92a")]
pub struct UpdateRewardMapping {
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

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, strategy, global_config, pool, reward_mint, reward_vault, base_vault_authority, token_infos, system_program, rent, token_program, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

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
