

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x0df5b467feb67904")]
pub struct Invest{
}

pub struct InvestInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Invest {
    type ArrangedAccounts = InvestInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let token_a_vault = accounts.get(3)?;
        let token_b_vault = accounts.get(4)?;
        let token_a_mint = accounts.get(5)?;
        let token_b_mint = accounts.get(6)?;
        let base_vault_authority = accounts.get(7)?;
        let pool = accounts.get(8)?;
        let token_a_token_program = accounts.get(9)?;
        let token_b_token_program = accounts.get(10)?;
        let memo_program = accounts.get(11)?;
        let token_program = accounts.get(12)?;
        let token_program2022 = accounts.get(13)?;
        let position = accounts.get(14)?;
        let raydium_protocol_position_or_base_vault_authority = accounts.get(15)?;
        let position_token_account = accounts.get(16)?;
        let pool_token_vault_a = accounts.get(17)?;
        let pool_token_vault_b = accounts.get(18)?;
        let tick_array_lower = accounts.get(19)?;
        let tick_array_upper = accounts.get(20)?;
        let scope_prices = accounts.get(21)?;
        let token_infos = accounts.get(22)?;
        let pool_program = accounts.get(23)?;
        let instruction_sysvar_account = accounts.get(24)?;
        let event_authority = accounts.get(25)?;

        Some(InvestInstructionAccounts {
            payer: payer.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            pool: pool.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            position: position.pubkey,
            raydium_protocol_position_or_base_vault_authority: raydium_protocol_position_or_base_vault_authority.pubkey,
            position_token_account: position_token_account.pubkey,
            pool_token_vault_a: pool_token_vault_a.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            scope_prices: scope_prices.pubkey,
            token_infos: token_infos.pubkey,
            pool_program: pool_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
            event_authority: event_authority.pubkey,
        })
    }
}