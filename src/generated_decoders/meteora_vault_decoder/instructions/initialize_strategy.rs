use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xd0779091b23969fc")]
pub struct InitializeStrategy {
    pub bumps: StrategyBumps,
    pub strategy_type: StrategyType,
}

pub struct InitializeStrategyInstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub strategy_program: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub collateral_vault: solana_sdk::pubkey::Pubkey,
    pub collateral_mint: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub rent: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeStrategy {
    type ArrangedAccounts = InitializeStrategyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let strategy_program = accounts.get(1)?;
        let strategy = accounts.get(2)?;
        let reserve = accounts.get(3)?;
        let collateral_vault = accounts.get(4)?;
        let collateral_mint = accounts.get(5)?;
        let admin = accounts.get(6)?;
        let system_program = accounts.get(7)?;
        let rent = accounts.get(8)?;
        let token_program = accounts.get(9)?;

        Some(InitializeStrategyInstructionAccounts {
            vault: vault.pubkey,
            strategy_program: strategy_program.pubkey,
            strategy: strategy.pubkey,
            reserve: reserve.pubkey,
            collateral_vault: collateral_vault.pubkey,
            collateral_mint: collateral_mint.pubkey,
            admin: admin.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
