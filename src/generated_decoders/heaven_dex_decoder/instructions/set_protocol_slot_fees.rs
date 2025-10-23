use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb552130f7ecd98f2")]
pub struct SetProtocolSlotFees {
    pub version: u16,
    pub fee_type: FeeType,
    pub slot_fees: SlotFeeBracketsParams,
}

pub struct SetProtocolSlotFeesInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetProtocolSlotFees {
    type ArrangedAccounts = SetProtocolSlotFeesInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let protocol_config_state = accounts.get(1)?;
        let protocol_owner_state = accounts.get(2)?;

        Some(SetProtocolSlotFeesInstructionAccounts {
            owner: owner.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
        })
    }
}
