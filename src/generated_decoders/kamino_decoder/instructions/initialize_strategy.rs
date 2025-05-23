use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd0779091b23969fc")]
pub struct InitializeStrategy {
    pub strategy_type: u64,
    pub token_a_collateral_id: u64,
    pub token_b_collateral_id: u64,
}

pub struct InitializeStrategyInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub shares_mint_authority: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeStrategy {
    type ArrangedAccounts = InitializeStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let admin_authority = accounts.get(0)?;
        let global_config = accounts.get(1)?;
        let pool = accounts.get(2)?;
        let token_a_mint = accounts.get(3)?;
        let token_b_mint = accounts.get(4)?;
        let token_a_vault = accounts.get(5)?;
        let token_b_vault = accounts.get(6)?;
        let base_vault_authority = accounts.get(7)?;
        let shares_mint = accounts.get(8)?;
        let shares_mint_authority = accounts.get(9)?;
        let token_infos = accounts.get(10)?;
        let system_program = accounts.get(11)?;
        let rent = accounts.get(12)?;
        let token_program = accounts.get(13)?;
        let token_a_token_program = accounts.get(14)?;
        let token_b_token_program = accounts.get(15)?;
        let strategy = accounts.get(16)?;

        Some(InitializeStrategyInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            shares_mint: shares_mint.pubkey,
            shares_mint_authority: shares_mint_authority.pubkey,
            token_infos: token_infos.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            strategy: strategy.pubkey,
        })
    }
}
