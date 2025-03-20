

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x816fae0c0a3c95c1")]
pub struct FlashSwapUnevenVaultsStart{
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

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let swapper = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let token_a_vault = accounts.get(3)?;
        let token_b_vault = accounts.get(4)?;
        let token_a_ata = accounts.get(5)?;
        let token_b_ata = accounts.get(6)?;
        let base_vault_authority = accounts.get(7)?;
        let pool = accounts.get(8)?;
        let position = accounts.get(9)?;
        let scope_prices = accounts.get(10)?;
        let token_infos = accounts.get(11)?;
        let tick_array_lower = accounts.get(12)?;
        let tick_array_upper = accounts.get(13)?;
        let token_a_mint = accounts.get(14)?;
        let token_b_mint = accounts.get(15)?;
        let token_a_token_program = accounts.get(16)?;
        let token_b_token_program = accounts.get(17)?;
        let instruction_sysvar_account = accounts.get(18)?;
        let consensus_account = accounts.get(19)?;

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