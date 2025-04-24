use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x151b891de295dd64")]
pub struct SetFlashLoanFee {
    pub flash_loan_fee: FlashLoanFee,
}

pub struct SetFlashLoanFeeInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub fee_authority: solana_sdk::pubkey::Pubkey,
    pub pool_account: solana_sdk::pubkey::Pubkey,
    pub flash_loan_fee_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFlashLoanFee {
    type ArrangedAccounts = SetFlashLoanFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let fee_authority = accounts.get(1)?;
        let pool_account = accounts.get(2)?;
        let flash_loan_fee_account = accounts.get(3)?;
        let system_program = accounts.get(4)?;

        Some(SetFlashLoanFeeInstructionAccounts {
            payer: payer.pubkey,
            fee_authority: fee_authority.pubkey,
            pool_account: pool_account.pubkey,
            flash_loan_fee_account: flash_loan_fee_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
