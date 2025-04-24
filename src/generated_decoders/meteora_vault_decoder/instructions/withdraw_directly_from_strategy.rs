use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc98d922ead74c616")]
pub struct WithdrawDirectlyFromStrategy {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
}

pub struct WithdrawDirectlyFromStrategyInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub strategy_program: solana_sdk::pubkey::Pubkey,
    pub collateral_vault: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub user_token: solana_sdk::pubkey::Pubkey,
    pub user_lp: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawDirectlyFromStrategy {
    type ArrangedAccounts = WithdrawDirectlyFromStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let reserve = accounts.get(2)?;
        let strategy_program = accounts.get(3)?;
        let collateral_vault = accounts.get(4)?;
        let token_vault = accounts.get(5)?;
        let lp_mint = accounts.get(6)?;
        let fee_vault = accounts.get(7)?;
        let user_token = accounts.get(8)?;
        let user_lp = accounts.get(9)?;
        let user = accounts.get(10)?;
        let token_program = accounts.get(11)?;

        Some(WithdrawDirectlyFromStrategyInstructionAccounts {
            vault: vault.pubkey,
            strategy: strategy.pubkey,
            reserve: reserve.pubkey,
            strategy_program: strategy_program.pubkey,
            collateral_vault: collateral_vault.pubkey,
            token_vault: token_vault.pubkey,
            lp_mint: lp_mint.pubkey,
            fee_vault: fee_vault.pubkey,
            user_token: user_token.pubkey,
            user_lp: user_lp.pubkey,
            user: user.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
