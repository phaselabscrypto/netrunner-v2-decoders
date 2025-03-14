

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x8a68d0947e23c30e")]
pub struct RemoveStrategy2{
    pub max_admin_pay_amount: u64,
}

pub struct RemoveStrategy2InstructionAccounts {
    pub vault: solana_sdk::pubkey::Pubkey,
    pub strategy: solana_sdk::pubkey::Pubkey,
    pub strategy_program: solana_sdk::pubkey::Pubkey,
    pub collateral_vault: solana_sdk::pubkey::Pubkey,
    pub reserve: solana_sdk::pubkey::Pubkey,
    pub token_vault: solana_sdk::pubkey::Pubkey,
    pub token_admin_advance_payment: solana_sdk::pubkey::Pubkey,
    pub token_vault_advance_payment: solana_sdk::pubkey::Pubkey,
    pub fee_vault: solana_sdk::pubkey::Pubkey,
    pub lp_mint: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub admin: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveStrategy2 {
    type ArrangedAccounts = RemoveStrategy2InstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let vault = accounts.get(0)?;
        let strategy = accounts.get(1)?;
        let strategy_program = accounts.get(2)?;
        let collateral_vault = accounts.get(3)?;
        let reserve = accounts.get(4)?;
        let token_vault = accounts.get(5)?;
        let token_admin_advance_payment = accounts.get(6)?;
        let token_vault_advance_payment = accounts.get(7)?;
        let fee_vault = accounts.get(8)?;
        let lp_mint = accounts.get(9)?;
        let token_program = accounts.get(10)?;
        let admin = accounts.get(11)?;

        Some(RemoveStrategy2InstructionAccounts {
            vault: vault.pubkey,
            strategy: strategy.pubkey,
            strategy_program: strategy_program.pubkey,
            collateral_vault: collateral_vault.pubkey,
            reserve: reserve.pubkey,
            token_vault: token_vault.pubkey,
            token_admin_advance_payment: token_admin_advance_payment.pubkey,
            token_vault_advance_payment: token_vault_advance_payment.pubkey,
            fee_vault: fee_vault.pubkey,
            lp_mint: lp_mint.pubkey,
            token_program: token_program.pubkey,
            admin: admin.pubkey,
        })
    }
}