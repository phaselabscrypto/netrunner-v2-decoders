use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x38f7aaf659dd86c8")]
pub struct CloseStrategy {}

pub struct CloseStrategyInstructionAccounts {
    pub admin_authority: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub old_position_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_position_mint_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_position_token_account_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_tick_array_lower_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub old_tick_array_upper_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub user_token_a_ata: solana_sdk::pubkey::Pubkey,
    pub user_token_b_ata: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub reward0_vault: solana_sdk::pubkey::Pubkey,
    pub reward1_vault: solana_sdk::pubkey::Pubkey,
    pub reward2_vault: solana_sdk::pubkey::Pubkey,
    pub kamino_reward0_vault: solana_sdk::pubkey::Pubkey,
    pub kamino_reward1_vault: solana_sdk::pubkey::Pubkey,
    pub kamino_reward2_vault: solana_sdk::pubkey::Pubkey,
    pub user_reward0_ata: solana_sdk::pubkey::Pubkey,
    pub user_reward1_ata: solana_sdk::pubkey::Pubkey,
    pub user_reward2_ata: solana_sdk::pubkey::Pubkey,
    pub user_kamino_reward0_ata: solana_sdk::pubkey::Pubkey,
    pub user_kamino_reward1_ata: solana_sdk::pubkey::Pubkey,
    pub user_kamino_reward2_ata: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub system: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseStrategy {
    type ArrangedAccounts = CloseStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin_authority, strategy, old_position_or_base_vault_authority, old_position_mint_or_base_vault_authority, old_position_token_account_or_base_vault_authority, old_tick_array_lower_or_base_vault_authority, old_tick_array_upper_or_base_vault_authority, pool, token_a_vault, token_b_vault, user_token_a_ata, user_token_b_ata, token_a_mint, token_b_mint, reward0_vault, reward1_vault, reward2_vault, kamino_reward0_vault, kamino_reward1_vault, kamino_reward2_vault, user_reward0_ata, user_reward1_ata, user_reward2_ata, user_kamino_reward0_ata, user_kamino_reward1_ata, user_kamino_reward2_ata, base_vault_authority, pool_program, token_program, token_a_token_program, token_b_token_program, system, event_authority, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(CloseStrategyInstructionAccounts {
            admin_authority: admin_authority.pubkey,
            strategy: strategy.pubkey,
            old_position_or_base_vault_authority: old_position_or_base_vault_authority.pubkey,
            old_position_mint_or_base_vault_authority: old_position_mint_or_base_vault_authority
                .pubkey,
            old_position_token_account_or_base_vault_authority:
                old_position_token_account_or_base_vault_authority.pubkey,
            old_tick_array_lower_or_base_vault_authority:
                old_tick_array_lower_or_base_vault_authority.pubkey,
            old_tick_array_upper_or_base_vault_authority:
                old_tick_array_upper_or_base_vault_authority.pubkey,
            pool: pool.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            user_token_a_ata: user_token_a_ata.pubkey,
            user_token_b_ata: user_token_b_ata.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            reward0_vault: reward0_vault.pubkey,
            reward1_vault: reward1_vault.pubkey,
            reward2_vault: reward2_vault.pubkey,
            kamino_reward0_vault: kamino_reward0_vault.pubkey,
            kamino_reward1_vault: kamino_reward1_vault.pubkey,
            kamino_reward2_vault: kamino_reward2_vault.pubkey,
            user_reward0_ata: user_reward0_ata.pubkey,
            user_reward1_ata: user_reward1_ata.pubkey,
            user_reward2_ata: user_reward2_ata.pubkey,
            user_kamino_reward0_ata: user_kamino_reward0_ata.pubkey,
            user_kamino_reward1_ata: user_kamino_reward1_ata.pubkey,
            user_kamino_reward2_ata: user_kamino_reward2_ata.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            pool_program: pool_program.pubkey,
            token_program: token_program.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            system: system.pubkey,
            event_authority: event_authority.pubkey,
        })
    }
}
