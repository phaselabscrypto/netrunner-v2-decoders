use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcceaccdb065b60f1")]
pub struct OpenLiquidityPosition {
    pub tick_lower_index: i64,
    pub tick_upper_index: i64,
    pub bump: u8,
}

pub struct OpenLiquidityPositionInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub position_mint: solana_sdk::pubkey::Pubkey,
    pub position_metadata_account: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub system: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub old_tick_array_lower_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_tick_array_upper_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_position_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_position_mint_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_position_token_account_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub consensus_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenLiquidityPosition {
    type ArrangedAccounts = OpenLiquidityPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, strategy, global_config, pool, tick_array_lower, tick_array_upper, base_vault_authority, position, position_mint, position_metadata_account, position_token_account, rent, system, token_program, token_program2022, token_a_token_program, token_b_token_program, memo_program, associated_token_program, pool_program, old_tick_array_lower_or_base_vault_authority, old_tick_array_upper_or_base_vault_authority, old_position_or_base_vault_authority, old_position_mint_or_base_vault_authority, old_position_token_account_or_base_vault_authority, token_a_vault, token_b_vault, token_a_mint, token_b_mint, pool_token_vault_a, pool_token_vault_b, scope_prices, token_infos, event_authority, consensus_account, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(OpenLiquidityPositionInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            position: position.pubkey,
            position_mint: position_mint.pubkey,
            position_metadata_account: position_metadata_account.pubkey,
            position_token_account: position_token_account.pubkey,
            rent: rent.pubkey,
            system: system.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            pool_program: pool_program.pubkey,
            old_tick_array_lower_or_base_vault_authority:
                old_tick_array_lower_or_base_vault_authority.pubkey,
            old_tick_array_upper_or_base_vault_authority:
                old_tick_array_upper_or_base_vault_authority.pubkey,
            old_position_or_base_vault_authority: old_position_or_base_vault_authority.pubkey,
            old_position_mint_or_base_vault_authority: old_position_mint_or_base_vault_authority
                .pubkey,
            old_position_token_account_or_base_vault_authority:
                old_position_token_account_or_base_vault_authority.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            pool_token_vault_a: pool_token_vault_a.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            scope_prices: scope_prices.pubkey,
            token_infos: token_infos.pubkey,
            event_authority: event_authority.pubkey,
            consensus_account: consensus_account.pubkey,
        })
    }
}
