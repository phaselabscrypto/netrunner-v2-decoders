use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x49e2f8d705c5d3e5")]
pub struct EmergencySwap {
    pub a_to_b: bool,
    pub target_limit_bps: u64,
}

pub struct EmergencySwapInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array0: solana_sdk::pubkey::Pubkey,
    pub tick_array1: solana_sdk::pubkey::Pubkey,
    pub tick_array2: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for EmergencySwap {
    type ArrangedAccounts = EmergencySwapInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, strategy, global_config, token_a_mint, token_b_mint, token_a_vault, token_b_vault, base_vault_authority, pool, position, pool_token_vault_a, pool_token_vault_b, tick_array0, tick_array1, tick_array2, oracle, pool_program, scope_prices, token_infos, token_a_token_program, token_b_token_program, memo_program, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(EmergencySwapInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            pool_token_vault_a: pool_token_vault_a.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            tick_array0: tick_array0.pubkey,
            tick_array1: tick_array1.pubkey,
            tick_array2: tick_array2.pubkey,
            oracle: oracle.pubkey,
            pool_program: pool_program.pubkey,
            scope_prices: scope_prices.pubkey,
            token_infos: token_infos.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
        })
    }
}
