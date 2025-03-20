

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xf223c68952e1f2b6")]
pub struct Deposit{
    pub token_max_a: u64,
    pub token_max_b: u64,
}

pub struct DepositInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_a_ata: solana_sdk::pubkey::Pubkey,
    pub token_b_ata: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub user_shares_ata: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub shares_mint_authority: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let user = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let pool = accounts.get(3)?;
        let position = accounts.get(4)?;
        let tick_array_lower = accounts.get(5)?;
        let tick_array_upper = accounts.get(6)?;
        let token_a_vault = accounts.get(7)?;
        let token_b_vault = accounts.get(8)?;
        let base_vault_authority = accounts.get(9)?;
        let token_a_ata = accounts.get(10)?;
        let token_b_ata = accounts.get(11)?;
        let token_a_mint = accounts.get(12)?;
        let token_b_mint = accounts.get(13)?;
        let user_shares_ata = accounts.get(14)?;
        let shares_mint = accounts.get(15)?;
        let shares_mint_authority = accounts.get(16)?;
        let scope_prices = accounts.get(17)?;
        let token_infos = accounts.get(18)?;
        let token_program = accounts.get(19)?;
        let token_a_token_program = accounts.get(20)?;
        let token_b_token_program = accounts.get(21)?;
        let instruction_sysvar_account = accounts.get(22)?;

        Some(DepositInstructionAccounts {
            user: user.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            token_a_ata: token_a_ata.pubkey,
            token_b_ata: token_b_ata.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            user_shares_ata: user_shares_ata.pubkey,
            shares_mint: shares_mint.pubkey,
            shares_mint_authority: shares_mint_authority.pubkey,
            scope_prices: scope_prices.pubkey,
            token_infos: token_infos.pubkey,
            token_program: token_program.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}