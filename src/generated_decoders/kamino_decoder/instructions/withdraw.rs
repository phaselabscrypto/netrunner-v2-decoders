use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb712469c946da122")]
pub struct Withdraw {
    pub shares_amount: u64,
}

pub struct WithdrawInstructionAccounts {
    pub user: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub global_config: solana_sdk::pubkey::Pubkey,
    pub pool: solana_sdk::pubkey::Pubkey,
    pub position: solana_sdk::pubkey::Pubkey,
    pub tick_array_lower: solana_sdk::pubkey::Pubkey,
    pub tick_array_upper: solana_sdk::pubkey::Pubkey,
    pub token_a_vault: solana_sdk::pubkey::Pubkey,
    pub token_b_vault: solana_sdk::pubkey::Pubkey,
    pub base_vault_authority: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_a: solana_sdk::pubkey::Pubkey,
    pub pool_token_vault_b: solana_sdk::pubkey::Pubkey,
    pub token_a_ata: solana_sdk::pubkey::Pubkey,
    pub token_b_ata: solana_sdk::pubkey::Pubkey,
    pub token_a_mint: solana_sdk::pubkey::Pubkey,
    pub token_b_mint: solana_sdk::pubkey::Pubkey,
    pub user_shares_ata: solana_sdk::pubkey::Pubkey,
    pub shares_mint: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_a_vault: solana_sdk::pubkey::Pubkey,
    pub treasury_fee_token_b_vault: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_program2022: solana_sdk::pubkey::Pubkey,
    pub token_a_token_program: solana_sdk::pubkey::Pubkey,
    pub token_b_token_program: solana_sdk::pubkey::Pubkey,
    pub memo_program: solana_sdk::pubkey::Pubkey,
    pub position_token_account: solana_sdk::pubkey::Pubkey,
    pub pool_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, strategy, global_config, pool, position, tick_array_lower, tick_array_upper, token_a_vault, token_b_vault, base_vault_authority, pool_token_vault_a, pool_token_vault_b, token_a_ata, token_b_ata, token_a_mint, token_b_mint, user_shares_ata, shares_mint, treasury_fee_token_a_vault, treasury_fee_token_b_vault, token_program, token_program2022, token_a_token_program, token_b_token_program, memo_program, position_token_account, pool_program, instruction_sysvar_account, event_authority, _remaining @ ..] =
            accounts.as_slice()
        else {
            return None;
        };

        Some(WithdrawInstructionAccounts {
            user: user.pubkey,
            strategy: strategy.pubkey,
            global_config: global_config.pubkey,
            pool: pool.pubkey,
            position: position.pubkey,
            tick_array_lower: tick_array_lower.pubkey,
            tick_array_upper: tick_array_upper.pubkey,
            token_a_vault: token_a_vault.pubkey,
            token_b_vault: token_b_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            pool_token_vault_a: pool_token_vault_a.pubkey,
            pool_token_vault_b: pool_token_vault_b.pubkey,
            token_a_ata: token_a_ata.pubkey,
            token_b_ata: token_b_ata.pubkey,
            token_a_mint: token_a_mint.pubkey,
            token_b_mint: token_b_mint.pubkey,
            user_shares_ata: user_shares_ata.pubkey,
            shares_mint: shares_mint.pubkey,
            treasury_fee_token_a_vault: treasury_fee_token_a_vault.pubkey,
            treasury_fee_token_b_vault: treasury_fee_token_b_vault.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
            token_a_token_program: token_a_token_program.pubkey,
            token_b_token_program: token_b_token_program.pubkey,
            memo_program: memo_program.pubkey,
            position_token_account: position_token_account.pubkey,
            pool_program: pool_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
            event_authority: event_authority.pubkey,
        })
    }
}
