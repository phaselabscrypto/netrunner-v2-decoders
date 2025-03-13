use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x816fae0c0a3c95c1")]
pub struct FlashSwapUnevenVaultsStart {
    pub amount: u64,
    pub a_to_b: bool,
}

pub struct FlashSwapUnevenVaultsStartInstructionAccounts {
    pub swapper: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub token_a_ata: solana_sdk::pubkey::Pubkey,
    pub token_b_ata: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
    pub consensus_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for FlashSwapUnevenVaultsStart {
    type ArrangedAccounts = FlashSwapUnevenVaultsStartInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let [swapper, strategy, global_config, token_a_vault, token_b_vault, token_a_ata, token_b_ata, base_vault_authority, pool, position, scope_prices, token_infos, tick_array_lower, tick_array_upper, token_a_mint, token_b_mint, token_a_token_program, token_b_token_program, instruction_sysvar_account, consensus_account, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(FlashSwapUnevenVaultsStartInstructionAccounts {
            swapper: swapper.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            token_a_ata: token_a_ata.pubkey,
            token_b_ata: token_b_ata.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            scope_prices: scope_prices.pubkey,
            token_infos: token_infos.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
            consensus_account: consensus_account.pubkey,
        })
    }
}
