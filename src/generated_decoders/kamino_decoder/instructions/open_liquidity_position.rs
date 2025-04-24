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
        let admin_authority = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let pool = accounts.get(3)?;
        let tick_array_lower = accounts.get(4)?;
        let tick_array_upper = accounts.get(5)?;
        let base_vault_authority = accounts.get(6)?;
        let position = accounts.get(7)?;
        let position_mint = accounts.get(8)?;
        let position_metadata_account = accounts.get(9)?;
        let position_token_account = accounts.get(10)?;
        let rent = accounts.get(11)?;
        let system = accounts.get(12)?;
        let token_program = accounts.get(13)?;
        let token_program2022 = accounts.get(14)?;
        let token_a_token_program = accounts.get(15)?;
        let token_b_token_program = accounts.get(16)?;
        let memo_program = accounts.get(17)?;
        let associated_token_program = accounts.get(18)?;
        let pool_program = accounts.get(19)?;
        let old_tick_array_lower_or_base_vault_authority = accounts.get(20)?;
        let old_tick_array_upper_or_base_vault_authority = accounts.get(21)?;
        let old_position_or_base_vault_authority = accounts.get(22)?;
        let old_position_mint_or_base_vault_authority = accounts.get(23)?;
        let old_position_token_account_or_base_vault_authority = accounts.get(24)?;
        let token_a_vault = accounts.get(25)?;
        let token_b_vault = accounts.get(26)?;
        let token_a_mint = accounts.get(27)?;
        let token_b_mint = accounts.get(28)?;
        let pool_token_vault_a = accounts.get(29)?;
        let pool_token_vault_b = accounts.get(30)?;
        let scope_prices = accounts.get(31)?;
        let token_infos = accounts.get(32)?;
        let event_authority = accounts.get(33)?;
        let consensus_account = accounts.get(34)?;

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
