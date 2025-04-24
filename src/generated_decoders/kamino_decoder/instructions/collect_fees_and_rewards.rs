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
        let user = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let base_vault_authority = accounts.get(3)?;
        let pool = accounts.get(4)?;
        let tick_array_lower = accounts.get(5)?;
        let tick_array_upper = accounts.get(6)?;
        let position = accounts.get(7)?;
        let raydium_protocol_position_or_base_vault_authority = accounts.get(8)?;
        let position_token_account = accounts.get(9)?;
        let token_a_vault = accounts.get(10)?;
        let pool_token_vault_a = accounts.get(11)?;
        let token_b_vault = accounts.get(12)?;
        let pool_token_vault_b = accounts.get(13)?;
        let treasury_fee_token_a_vault = accounts.get(14)?;
        let treasury_fee_token_b_vault = accounts.get(15)?;
        let treasury_fee_vault_authority = accounts.get(16)?;
        let reward0_vault = accounts.get(17)?;
        let reward1_vault = accounts.get(18)?;
        let reward2_vault = accounts.get(19)?;
        let pool_reward_vault0 = accounts.get(20)?;
        let pool_reward_vault1 = accounts.get(21)?;
        let pool_reward_vault2 = accounts.get(22)?;
        let token_a_mint = accounts.get(23)?;
        let token_b_mint = accounts.get(24)?;
        let token_a_token_program = accounts.get(25)?;
        let token_b_token_program = accounts.get(26)?;
        let memo_program = accounts.get(27)?;
        let token_program = accounts.get(28)?;
        let token_program2022 = accounts.get(29)?;
        let pool_program = accounts.get(30)?;
        let instruction_sysvar_account = accounts.get(31)?;
        let event_authority = accounts.get(32)?;

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
