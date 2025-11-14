use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcce291cde825038c")]
pub struct CloseBorrowPosition {
    pub params: CloseBorrowPositionParams,
}

pub struct CloseBorrowPositionInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub borrow_position: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseBorrowPosition {
    type ArrangedAccounts = CloseBorrowPositionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, borrow_position, system_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseBorrowPositionInstructionAccounts {
            owner: owner.pubkey,
            borrow_position: borrow_position.pubkey,
            system_program: system_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
