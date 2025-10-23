use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xc5617b36dda80b87")]
pub struct UpdateProtocolConfig {
    pub version: u16,
    pub params: ProtocolConfigParams,
}

pub struct UpdateProtocolConfigInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub protocol_config_state: solana_sdk::pubkey::Pubkey,
    pub protocol_owner_state: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateProtocolConfig {
    type ArrangedAccounts = UpdateProtocolConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let protocol_config_state = accounts.get(1)?;
        let protocol_owner_state = accounts.get(2)?;

        Some(UpdateProtocolConfigInstructionAccounts {
            owner: owner.pubkey,
            protocol_config_state: protocol_config_state.pubkey,
            protocol_owner_state: protocol_owner_state.pubkey,
        })
    }
}
