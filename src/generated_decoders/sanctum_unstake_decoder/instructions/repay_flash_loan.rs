

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x77ef122dc26b1fee")]
pub struct RepayFlashLoan{
}

pub struct RepayFlashLoanInstructionAccounts {
    pub repayer: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub pool_sol_reserves: solana_sdk::pubkey::Pubkey,
    pub flash_account: solana_sdk::pubkey::Pubkey,
    pub flash_loan_fee_account: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_account: solana_sdk::pubkey::Pubkey,
    pub protocol_fee_destination: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepayFlashLoan {
    type ArrangedAccounts = RepayFlashLoanInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let repayer = accounts.get(0)?;
        let pool_account = accounts.get(1)?;
        let pool_sol_reserves = accounts.get(2)?;
        let flash_account = accounts.get(3)?;
        let flash_loan_fee_account = accounts.get(4)?;
        let protocol_fee_account = accounts.get(5)?;
        let protocol_fee_destination = accounts.get(6)?;
        let system_program = accounts.get(7)?;

        Some(RepayFlashLoanInstructionAccounts {
            repayer: repayer.pubkey,
            pool_account: pool_account.pubkey,
            pool_sol_reserves: pool_sol_reserves.pubkey,
            flash_account: flash_account.pubkey,
            flash_loan_fee_account: flash_loan_fee_account.pubkey,
            protocol_fee_account: protocol_fee_account.pubkey,
            protocol_fee_destination: protocol_fee_destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}