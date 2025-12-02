use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x905443271b19ca8d")]
pub struct CancelTriggerOrder {
    pub params: CancelTriggerOrderParams,
}

pub struct CancelTriggerOrderInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelTriggerOrder {
    type ArrangedAccounts = CancelTriggerOrderInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, order, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelTriggerOrderInstructionAccounts {
            owner: owner.pubkey,
            order: order.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
