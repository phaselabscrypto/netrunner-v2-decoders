use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x76868fc0bc158311")]
pub struct SingleTokenDepositAndInvestWithMin {
    pub token_a_min_post_deposit_balance: u64,
    pub token_b_min_post_deposit_balance: u64,
}

pub struct SingleTokenDepositAndInvestWithMinInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
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
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SingleTokenDepositAndInvestWithMin {
    type ArrangedAccounts = SingleTokenDepositAndInvestWithMinInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, strategy, global_config, pool, position, raydium_protocol_position_or_base_vault_authority, position_token_account, token_a_vault, token_b_vault, pool_token_vault_a, pool_token_vault_b, tick_array_lower, tick_array_upper, base_vault_authority, token_a_ata, token_b_ata, token_a_mint, token_b_mint, user_shares_ata, shares_mint, shares_mint_authority, scope_prices, token_infos, token_program, token_program2022, token_a_token_program, token_b_token_program, memo_program, pool_program, instruction_sysvar_account, event_authority, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(SingleTokenDepositAndInvestWithMinInstructionAccounts {
            user: user.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            raydium_protocol_position_or_base_vault_authority:
                raydium_protocol_position_or_base_vault_authority.pubkey,
            position_token_account: position_token_account.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            pool_token_vault_a: pool_token_vault_a.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
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
            token_program2022: token_program2022.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
            pool_program: pool_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
            event_authority: event_authority.pubkey,
        })
    }
}
