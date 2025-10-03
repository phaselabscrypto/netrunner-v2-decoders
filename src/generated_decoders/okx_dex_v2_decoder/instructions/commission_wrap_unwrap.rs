use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c499c47e9acbdc5")]
pub struct CommissionWrapUnwrap {
    pub data: CommissionWrapUnwrapArgs,
    pub order_id: u64,
}

pub struct CommissionWrapUnwrapInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub payer_wsol_account: solana_sdk::pubkey::Pubkey,
    pub wsol_mint: solana_sdk::pubkey::Pubkey,
    pub temp_wsol_account: solana_sdk::pubkey::Pubkey,
    pub commission_sol_account: solana_sdk::pubkey::Pubkey,
    pub commission_wsol_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CommissionWrapUnwrap {
    type ArrangedAccounts = CommissionWrapUnwrapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let payer_wsol_account = accounts.get(1)?;
        let wsol_mint = accounts.get(2)?;
        let temp_wsol_account = accounts.get(3)?;
        let commission_sol_account = accounts.get(4)?;
        let commission_wsol_account = accounts.get(5)?;
        let system_program = accounts.get(6)?;
        let token_program = accounts.get(7)?;

        Some(CommissionWrapUnwrapInstructionAccounts {
            payer: payer.pubkey,
            payer_wsol_account: payer_wsol_account.pubkey,
            wsol_mint: wsol_mint.pubkey,
            temp_wsol_account: temp_wsol_account.pubkey,
            commission_sol_account: commission_sol_account.pubkey,
            commission_wsol_account: commission_wsol_account.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
