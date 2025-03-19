use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x71124b08b61f69ba")]
pub struct CollectFeesAndRewards {}

pub struct CollectFeesAndRewardsInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub raydium_protocol_position_or_base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_a_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_b_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault_authority: solana_sdk::pubkey::Pubkey,
    pub reward0_vault: solana_sdk::pubkey::Pubkey,
    pub reward1_vault: solana_sdk::pubkey::Pubkey,
    pub reward2_vault: solana_sdk::pubkey::Pubkey,
    pub pool_reward_vault0: solana_sdk::pubkey::Pubkey,
    pub pool_reward_vault1: solana_sdk::pubkey::Pubkey,
    pub pool_reward_vault2: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFeesAndRewards {
    type ArrangedAccounts = CollectFeesAndRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, strategy, global_config, base_vault_authority, pool, tick_array_lower, tick_array_upper, position, raydium_protocol_position_or_base_vault_authority, position_token_account, token_a_vault, pool_token_vault_a, token_b_vault, pool_token_vault_b, treasury_fee_token_a_vault, treasury_fee_token_b_vault, treasury_fee_vault_authority, reward0_vault, reward1_vault, reward2_vault, pool_reward_vault0, pool_reward_vault1, pool_reward_vault2, token_a_mint, token_b_mint, token_a_token_program, token_b_token_program, memo_program, token_program, token_program2022, pool_program, instruction_sysvar_account, event_authority, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(CollectFeesAndRewardsInstructionAccounts {
            user: user.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            pool: pool.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            position: position.pubkey,
            raydium_protocol_position_or_base_vault_authority:
                raydium_protocol_position_or_base_vault_authority.pubkey,
            position_token_account: position_token_account.pubkey,
            token_a_vault: token_a_vault.pubkey,
            pool_token_vault_a: pool_token_vault_a.pubkey,
            token_b_vault: token_b_vault.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            treasury_fee_token_a_vault: treasury_fee_token_a_vault.pubkey,
            treasury_fee_token_b_vault: treasury_fee_token_b_vault.pubkey,
            treasury_fee_vault_authority: treasury_fee_vault_authority.pubkey,
            reward0_vault: reward0_vault.pubkey,
            reward1_vault: reward1_vault.pubkey,
            reward2_vault: reward2_vault.pubkey,
            pool_reward_vault0: pool_reward_vault0.pubkey,
            pool_reward_vault1: pool_reward_vault1.pubkey,
            pool_reward_vault2: pool_reward_vault2.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            pool_program: pool_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
            event_authority: event_authority.pubkey,
        })
    }
}
