use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2349858b2037d58c")]
pub struct WithdrawByMmm {
    pub args: WithdrawByMMMArgs,
}

pub struct WithdrawByMmmInstructionAccounts {
    pub mmm_pool: solana_sdk::pubkey::Pubkey,
    pub to: solana_sdk::pubkey::Pubkey,
    pub escrow_payment_account: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawByMmm {
    type ArrangedAccounts = WithdrawByMmmInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mmm_pool = accounts.get(0)?;
        let to = accounts.get(1)?;
        let escrow_payment_account = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(WithdrawByMmmInstructionAccounts {
            mmm_pool: mmm_pool.pubkey,
            to: to.pubkey,
            escrow_payment_account: escrow_payment_account.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
