use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x826c2199e41fd8db")]
pub struct CancelAllTriggerOrders {}

pub struct CancelAllTriggerOrdersInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub order: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CancelAllTriggerOrders {
    type ArrangedAccounts = CancelAllTriggerOrdersInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, order, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CancelAllTriggerOrdersInstructionAccounts {
            position: position.pubkey,
            order: order.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
