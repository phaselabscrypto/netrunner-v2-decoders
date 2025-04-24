use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x52689806956f640d")]
pub struct SetDecompressibleState {
    pub decompressable_state: DecompressibleState,
}

pub struct SetDecompressibleStateInstructionAccounts {
    pub tree_authority: solana_sdk::pubkey::Pubkey,
    pub tree_creator: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetDecompressibleState {
    type ArrangedAccounts = SetDecompressibleStateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let tree_authority = accounts.get(0)?;
        let tree_creator = accounts.get(1)?;

        Some(SetDecompressibleStateInstructionAccounts {
            tree_authority: tree_authority.pubkey,
            tree_creator: tree_creator.pubkey,
        })
    }
}
