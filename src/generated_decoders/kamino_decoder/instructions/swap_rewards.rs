use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c29ac1ebe41ae5a")]
pub struct SwapRewards {
    pub token_a_in: u64,
    pub token_b_in: u64,
    pub reward_index: u64,
    pub reward_collateral_id: u64,
    pub min_collateral_token_out: u64,
}

pub struct SwapRewardsInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub reward_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_a_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_b_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_vault_authority: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub reward_mint: solana_sdk::pubkey::Pubkey,
    pub user_token_a_ata: solana_sdk::pubkey::Pubkey,
    pub user_token_b_ata: solana_sdk::pubkey::Pubkey,
    pub user_reward_token_account: solana_sdk::pubkey::Pubkey,
    pub scope_prices: solana_sdk::pubkey::Pubkey,
    pub token_infos: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub reward_token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapRewards {
    type ArrangedAccounts = SwapRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let user = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let global_config = accounts.get(2)?;
        let pool = accounts.get(3)?;
        let token_a_vault = accounts.get(4)?;
        let token_b_vault = accounts.get(5)?;
        let reward_vault = accounts.get(6)?;
        let base_vault_authority = accounts.get(7)?;
        let treasury_fee_token_a_vault = accounts.get(8)?;
        let treasury_fee_token_b_vault = accounts.get(9)?;
        let treasury_fee_vault_authority = accounts.get(10)?;
        let token_a_mint = accounts.get(11)?;
        let token_b_mint = accounts.get(12)?;
        let reward_mint = accounts.get(13)?;
        let user_token_a_ata = accounts.get(14)?;
        let user_token_b_ata = accounts.get(15)?;
        let user_reward_token_account = accounts.get(16)?;
        let scope_prices = accounts.get(17)?;
        let token_infos = accounts.get(18)?;
        let system_program = accounts.get(19)?;
        let token_a_token_program = accounts.get(20)?;
        let token_b_token_program = accounts.get(21)?;
        let reward_token_program = accounts.get(22)?;
        let instruction_sysvar_account = accounts.get(23)?;

        Some(SwapRewardsInstructionAccounts {
            user: user.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            reward_vault: reward_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            treasury_fee_token_a_vault: treasury_fee_token_a_vault.pubkey,
            treasury_fee_token_b_vault: treasury_fee_token_b_vault.pubkey,
            treasury_fee_vault_authority: treasury_fee_vault_authority.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            reward_mint: reward_mint.pubkey,
            user_token_a_ata: user_token_a_ata.pubkey,
            user_token_b_ata: user_token_b_ata.pubkey,
            user_reward_token_account: user_reward_token_account.pubkey,
            scope_prices: scope_prices.pubkey,
            token_infos: token_infos.pubkey,
            system_program: system_program.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            reward_token_program: reward_token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}
