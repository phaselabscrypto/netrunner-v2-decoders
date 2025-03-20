

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw{
    pub shares_amount: u64,
}

pub struct WithdrawInstructionAccounts {
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
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub token_a_ata: solana_sdk::pubkey::Pubkey,
    pub token_b_ata: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub user_shares_ata: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_a_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_b_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

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
        let pool_token_vault_a = accounts.get(10)?;
        let pool_token_vault_b = accounts.get(11)?;
        let token_a_ata = accounts.get(12)?;
        let token_b_ata = accounts.get(13)?;
        let token_a_mint = accounts.get(14)?;
        let token_b_mint = accounts.get(15)?;
        let user_shares_ata = accounts.get(16)?;
        let shares_mint = accounts.get(17)?;
        let treasury_fee_token_a_vault = accounts.get(18)?;
        let treasury_fee_token_b_vault = accounts.get(19)?;
        let token_program = accounts.get(20)?;
        let token_program2022 = accounts.get(21)?;
        let token_a_token_program = accounts.get(22)?;
        let token_b_token_program = accounts.get(23)?;
        let memo_program = accounts.get(24)?;
        let position_token_account = accounts.get(25)?;
        let pool_program = accounts.get(26)?;
        let instruction_sysvar_account = accounts.get(27)?;
        let event_authority = accounts.get(28)?;

        Some(WithdrawInstructionAccounts {
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
            pool_token_vault_a: pool_token_vault_a.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            token_a_ata: token_a_ata.pubkey,
            token_b_ata: token_b_ata.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            user_shares_ata: user_shares_ata.pubkey,
            shares_mint: shares_mint.pubkey,
            treasury_fee_token_a_vault: treasury_fee_token_a_vault.pubkey,
            treasury_fee_token_b_vault: treasury_fee_token_b_vault.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
            position_token_account: position_token_account.pubkey,
            pool_program: pool_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
            event_authority: event_authority.pubkey,
        })
    }
}